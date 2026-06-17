//! The windowed viewer — Galene running live (the pre-Tauri "Open Galene"
//! experience). A winit window + wgpu surface drive the real-time render loop:
//! each frame ticks the [`EnvironmentDriver`] and a [`FocusSession`], then
//! renders the scene via `SceneRenderer` to the surface.
//!
//! Controls: ↑/↓ richness · Space toggle work/break · Esc quit. The host (this
//! loop) supplies `time` and a per-frame `seed`, keeping the renderer's
//! per-frame path free of wall-clock/RNG (AV-006).

use std::sync::Arc;
use std::time::Instant;

use flowstate_core::{FocusSession, SessionEvent, SessionType, WorkBreakState};
use flowstate_visual::{EnvironmentDriver, GpuContext, Scene, SceneRenderer};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const RAINY_LIBRARY: &str = include_str!("../../environments/rainy_library.ron");

/// Per-window GPU + scene state, created once the window exists.
struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    gpu: GpuContext,
    renderer: SceneRenderer,
    driver: EnvironmentDriver,
    session: FocusSession,
    start: Instant,
    last: Instant,
    frame: u32,
}

impl State {
    fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));

        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let surface = instance
            .create_surface(window.clone())
            .expect("create surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("no compatible GPU adapter");
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("galene-viewer"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
        }))
        .expect("request device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: w,
            height: h,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let gpu = GpuContext {
            device,
            queue,
            adapter_info: adapter.get_info(),
        };

        let scene = Scene::from_ron(RAINY_LIBRARY).expect("scene parses");
        println!("Galene — {} on {}", scene.name, gpu.adapter_info.name);
        println!("Controls: ↑/↓ richness · Space work/break · Esc quit");

        let driver = EnvironmentDriver::new(scene);
        let renderer = SceneRenderer::new(&gpu, w, h, format, &driver.scene);
        let session = FocusSession::new(SessionType::default(), driver.scene.id.clone(), None);

        let now = Instant::now();
        Self {
            window,
            surface,
            config,
            gpu,
            renderer,
            driver,
            session,
            start: now,
            last: now,
            frame: 0,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.gpu.device, &self.config);
        // Layer targets are sized at construction → rebuild for the new size.
        self.renderer =
            SceneRenderer::new(&self.gpu, width, height, self.config.format, &self.driver.scene);
    }

    fn adjust_richness(&mut self, delta: f32) {
        self.driver.richness = (self.driver.richness + delta).clamp(0.0, 1.0);
        println!("richness = {:.2}", self.driver.richness);
    }

    fn toggle_state(&mut self) {
        let next = match self.driver.state {
            WorkBreakState::Work => WorkBreakState::Break,
            WorkBreakState::Break => WorkBreakState::Work,
        };
        self.driver.set_state(next);
        println!("→ {:?} (transitioning)", next);
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last).as_secs_f32();
        self.last = now;

        // A real Pomodoro ticks underneath; its boundaries drive the transition.
        match self.session.tick(dt) {
            SessionEvent::EnteredBreak { .. } => self.driver.set_state(WorkBreakState::Break),
            SessionEvent::EnteredWork => self.driver.set_state(WorkBreakState::Work),
            SessionEvent::None | SessionEvent::Completed => {}
        }
        self.driver.advance(dt);
    }

    fn render(&mut self) {
        use wgpu::CurrentSurfaceTexture;
        let frame = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(f) | CurrentSurfaceTexture::Suboptimal(f) => f,
            // Lost / outdated / timed out / occluded (e.g. mid-resize, minimized)
            // — reconfigure and skip this frame.
            _ => {
                self.surface.configure(&self.gpu.device, &self.config);
                return;
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let t = self.start.elapsed().as_secs_f32();
        self.renderer.render(&self.gpu, &self.driver, t, self.frame, &view);
        frame.present();
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Galene — The Rainy Library")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 540.0));
        let window = Arc::new(event_loop.create_window(attrs).expect("create window"));
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => event_loop.exit(),
                    PhysicalKey::Code(KeyCode::ArrowUp) => state.adjust_richness(0.05),
                    PhysicalKey::Code(KeyCode::ArrowDown) => state.adjust_richness(-0.05),
                    PhysicalKey::Code(KeyCode::Space) => state.toggle_state(),
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                state.update();
                state.render();
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}

/// Open the window and run the live render loop until the user quits.
pub fn run() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("run event loop");
}
