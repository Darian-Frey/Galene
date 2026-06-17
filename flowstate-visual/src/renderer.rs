//! Top-level render entry points.
//!
//! The windowed/surface render loop (driven by the app) is added later. For now
//! this provides **headless** single-frame renders to an RGBA8 buffer, which is
//! how the offscreen→DOF→composite path is exercised and tested without a window
//! (render-doc §11 steps 1–2). The render-doc §12 questions are resolved (D-011):
//! this stack is greenfield in Galene.

use crate::compositor::{CompositeLayer, Compositor, LAYER_FORMAT};
use crate::driver::EnvironmentDriver;
use crate::gpu::GpuContext;
use crate::layer::{BlendMode, ResolvedParams};
use crate::modules::{build_module, ModuleInit, VisualModule};
use crate::scene::Scene;

/// Build a `ModuleInit` for constructing modules that render into a layer target
/// (RGBA16F).
pub fn module_init(gpu: &GpuContext) -> ModuleInit<'_> {
    ModuleInit {
        device: &gpu.device,
        queue: &gpu.queue,
        target_format: LAYER_FORMAT,
    }
}

/// Render a back-to-front stack of layers through the compositor at
/// `width`×`height` and read the result back as tightly-packed RGBA8.
/// `modules`, `specs`, and `params` are parallel (index 0 = back layer).
pub fn render_layers_to_rgba8(
    gpu: &GpuContext,
    width: u32,
    height: u32,
    modules: &mut [&mut (dyn VisualModule + '_)],
    specs: &[CompositeLayer],
    params: &[ResolvedParams],
) -> Vec<u8> {
    let device = &gpu.device;
    let output_format = wgpu::TextureFormat::Rgba8Unorm;

    let output_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("render.output"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: output_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let output_view = output_tex.create_view(&wgpu::TextureViewDescriptor::default());

    let compositor = Compositor::new(device, width, height, output_format, specs);
    compositor.render_frame(gpu, modules, params, 0.0, 0, &output_view);

    read_texture_rgba8(gpu, &output_tex, width, height)
}

/// Render a single module as one full-resolution, sharp, normal-blend layer.
pub fn render_module_to_rgba8(
    gpu: &GpuContext,
    width: u32,
    height: u32,
    module: &mut dyn VisualModule,
    params: &ResolvedParams,
) -> Vec<u8> {
    let specs = [CompositeLayer {
        resolution_scale: 1.0,
        depth_blur: 0.0,
        blend: BlendMode::Normal,
    }];
    let mut modules: [&mut dyn VisualModule; 1] = [module];
    render_layers_to_rgba8(gpu, width, height, &mut modules, &specs, std::slice::from_ref(params))
}

/// Renders a whole scene: a compositor sized to the scene's layers plus the
/// runtime module per layer (built via [`build_module`]). Parameters come from
/// an [`EnvironmentDriver`] each frame, so the richness dial, work/break state,
/// and evolution cycle drive the visible output (render-doc §11 step 3).
///
/// Build this from the same scene the driver owns (`&driver.scene`) so the
/// per-layer modules line up with the driver's resolved parameters.
pub struct SceneRenderer {
    compositor: Compositor,
    modules: Vec<Box<dyn VisualModule>>,
}

impl SceneRenderer {
    pub fn new(
        gpu: &GpuContext,
        width: u32,
        height: u32,
        output_format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let specs: Vec<CompositeLayer> = scene
            .layers
            .iter()
            .map(|l| CompositeLayer {
                resolution_scale: l.resolution_scale,
                depth_blur: l.depth_blur,
                blend: l.blend,
            })
            .collect();
        let compositor = Compositor::new(&gpu.device, width, height, output_format, &specs);

        let init = module_init(gpu);
        let modules = scene
            .layers
            .iter()
            .map(|l| build_module(&l.module, &init))
            .collect();

        Self {
            compositor,
            modules,
        }
    }

    /// Render one frame, resolving each layer's parameters from `driver`.
    pub fn render(
        &mut self,
        gpu: &GpuContext,
        driver: &EnvironmentDriver,
        time_secs: f32,
        seed: u32,
        output_view: &wgpu::TextureView,
    ) {
        let params = driver.resolve_all();
        let mut mods: Vec<&mut dyn VisualModule> =
            self.modules.iter_mut().map(|b| b.as_mut()).collect();
        self.compositor
            .render_frame(gpu, &mut mods, &params, time_secs, seed, output_view);
    }
}

/// Render a scene through a [`SceneRenderer`] (built with `Rgba8Unorm` output)
/// and read it back as tightly-packed RGBA8.
pub fn render_scene_to_rgba8(
    gpu: &GpuContext,
    width: u32,
    height: u32,
    renderer: &mut SceneRenderer,
    driver: &EnvironmentDriver,
    time_secs: f32,
    seed: u32,
) -> Vec<u8> {
    let device = &gpu.device;
    let output_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("scene.output"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let output_view = output_tex.create_view(&wgpu::TextureViewDescriptor::default());
    renderer.render(gpu, driver, time_secs, seed, &output_view);
    read_texture_rgba8(gpu, &output_tex, width, height)
}

/// Copy an RGBA8 texture into a mappable buffer and read it back as
/// tightly-packed RGBA8 (`width*height*4` bytes; row padding removed).
pub(crate) fn read_texture_rgba8(
    gpu: &GpuContext,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let device = &gpu.device;
    let queue = &gpu.queue;

    let unpadded_bytes_per_row = width * 4;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("render.readback"),
        size: (padded_bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("render.copy"),
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    let slice = buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    rx.recv().expect("map callback").expect("buffer map");

    let mapped = slice.get_mapped_range();
    let mut out = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
    for row in 0..height {
        let start = (row * padded_bytes_per_row) as usize;
        let end = start + unpadded_bytes_per_row as usize;
        out.extend_from_slice(&mapped[start..end]);
    }
    drop(mapped);
    buffer.unmap();

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::shader_canvas::ShaderCanvasModule;

    #[test]
    fn shader_canvas_renders_a_vertical_gradient() {
        let Some(gpu) = GpuContext::new_headless() else {
            eprintln!("no GPU adapter — skipping render test");
            return;
        };

        let mut module = ShaderCanvasModule::new(&module_init(&gpu), "warm_interior_gradient");

        let mut params = ResolvedParams::new();
        params.insert("warmth".into(), 0.8);
        params.insert("darkness".into(), 0.2);

        let (w, h) = (64u32, 64u32);
        let px = render_module_to_rgba8(&gpu, w, h, &mut module, &params);
        assert_eq!(px.len() as u32, w * h * 4);

        let red_at = |row: u32| -> u8 {
            let x = w / 2;
            px[((row * w + x) * 4) as usize]
        };
        // The gradient warms toward the bottom → more red there than at the top.
        assert!(
            red_at(h - 2) > red_at(1),
            "expected a vertical gradient (bottom warmer than top)",
        );
        assert!(px.iter().any(|&b| b > 0), "frame is entirely black");
    }

    #[test]
    fn richness_dial_changes_the_rendered_scene() {
        let Some(gpu) = GpuContext::new_headless() else {
            eprintln!("no GPU adapter — skipping scene render test");
            return;
        };

        let scene =
            Scene::from_ron(include_str!("../../environments/rainy_library.ron")).unwrap();
        let mut driver = EnvironmentDriver::new(scene);
        let (w, h) = (96u32, 54u32);
        let mut sr = SceneRenderer::new(&gpu, w, h, wgpu::TextureFormat::Rgba8Unorm, &driver.scene);

        // Low richness in the work state.
        driver.richness = 0.05;
        driver.state = flowstate_core::WorkBreakState::Work;
        driver.state_blend = 0.0;
        let low = render_scene_to_rgba8(&gpu, w, h, &mut sr, &driver, 0.0, 0);

        // High richness in the break state.
        driver.richness = 1.0;
        driver.state = flowstate_core::WorkBreakState::Break;
        driver.state_blend = 1.0;
        let high = render_scene_to_rgba8(&gpu, w, h, &mut sr, &driver, 0.0, 0);

        assert_eq!(low.len(), high.len());
        assert!(low != high, "richness dial / state should change the image");
        assert!(high.iter().any(|&b| b > 0), "break frame is entirely black");
    }
}
