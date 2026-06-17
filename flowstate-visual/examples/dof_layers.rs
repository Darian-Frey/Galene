//! Two-layer depth-of-field demonstration (render-doc §11 step 2): a heavily
//! blurred background gradient with a sharp foreground over it, proving the
//! per-layer depth separation reads correctly.
//!
//!     cargo run -p flowstate-visual --example dof_layers
//!     # writes dof_layers.ppm (open with any image viewer)
//!
//! The foreground `Bars` module is defined here, in the example, which also
//! exercises the public `VisualModule` trait from outside the crate.

use std::collections::HashMap;
use std::io::Write;

use flowstate_visual::{
    module_init, render_layers_to_rgba8, BlendMode, CompositeLayer, FrameCtx, GpuContext,
    ModuleInit, VisualModule, FULLSCREEN_VS,
};

/// Vertical bars with transparent gaps. Two instances demonstrate depth
/// separation: narrow cool bars behind (blurred), wide warm bars in front (sharp).
struct Bars {
    pipeline: wgpu::RenderPipeline,
}

/// Narrow cool bars — the background layer (will be blurred).
const BARS_BG_FS: &str = r#"
@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let inside = step(fract(uv.x * 11.0), 0.5);   // 11 narrow bars
    let col = vec3<f32>(0.40, 0.52, 0.72);
    return vec4<f32>(col * inside, inside);
}
"#;

/// Wide warm bars, phase-shifted — the foreground layer (stays sharp).
const BARS_FG_FS: &str = r#"
@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let inside = step(fract(uv.x * 3.0 + 0.15), 0.4);  // 3 wide bars, big gaps
    let col = vec3<f32>(1.0, 0.62, 0.30);
    return vec4<f32>(col * inside, inside);
}
"#;

impl Bars {
    fn new(init: &ModuleInit, fragment: &str) -> Self {
        let device = init.device;
        let source = format!("{FULLSCREEN_VS}\n{fragment}");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bars"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bars.layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bars.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { pipeline }
    }
}

impl VisualModule for Bars {
    fn name(&self) -> &str {
        "Bars"
    }

    fn render(
        &mut self,
        _ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("bars.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.draw(0..3, 0..1);
    }
}

fn main() {
    let Some(gpu) = GpuContext::new_headless() else {
        eprintln!("No GPU adapter available — cannot render.");
        std::process::exit(1);
    };
    println!("Rendering on: {} [{:?}]", gpu.adapter_info.name, gpu.adapter_info.backend);

    let (w, h) = (512u32, 288u32);

    let mut background = Bars::new(&module_init(&gpu), BARS_BG_FS);
    let mut foreground = Bars::new(&module_init(&gpu), BARS_FG_FS);

    // Back layer: narrow cool bars, heavily blurred (and half-res — invisible
    // once blurred). Front layer: wide warm bars, sharp.
    let specs = [
        CompositeLayer {
            resolution_scale: 0.5,
            depth_blur: 0.9,
            blend: BlendMode::Normal,
        },
        CompositeLayer {
            resolution_scale: 1.0,
            depth_blur: 0.0,
            blend: BlendMode::Normal,
        },
    ];

    let params = [HashMap::new(), HashMap::new()];

    let mut modules: [&mut dyn VisualModule; 2] = [&mut background, &mut foreground];
    let rgba = render_layers_to_rgba8(&gpu, w, h, &mut modules, &specs, &params);

    let path = "dof_layers.ppm";
    let mut file = std::fs::File::create(path).expect("create output file");
    write!(file, "P6\n{w} {h}\n255\n").unwrap();
    let mut rgb = Vec::with_capacity((w * h * 3) as usize);
    for px in rgba.chunks_exact(4) {
        rgb.extend_from_slice(&px[0..3]);
    }
    file.write_all(&rgb).unwrap();

    println!("Wrote {path} ({w}×{h}) — sharp warm bars over blurred cool bars.");
}
