//! The shared visual-module system.
//!
//! Modules implement a single trait so a module is written once. The render-doc
//! §12 questions are resolved (DECISIONS D-011): there is no Synaesthesia
//! upstream, so Galene defines the canonical trait here, and Synaesthesia (when
//! built) conforms to it.
//!
//! Most modules (Particle System, Fluid Field, Geometric Field, Terrain, Voronoi,
//! Cellular Automata, Waveform Ribbon, Shader Canvas) are shared (render doc §4).
//! The two new primitives — [`glass_rain`] and [`volumetric_light`] — live here.

pub mod geometric_field;
pub mod glass_rain;
pub mod placeholder;
pub mod shader_canvas;
pub mod volumetric_light;

use serde::{Deserialize, Serialize};

pub use geometric_field::GeometricFieldModule;
pub use glass_rain::GlassRainModule;
pub use placeholder::PlaceholderModule;
pub use shader_canvas::ShaderCanvasModule;
pub use volumetric_light::{LightSource, VolumetricLightModule};

use crate::layer::ResolvedParams;

/// Names the module that draws a layer, plus its instance configuration, as it
/// appears in a scene's RON definition (render doc §7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleSpec {
    // Shared Synaesthesia modules
    ShaderCanvas { shader: String },
    GeometricField { preset: String },
    ParticleSystem { preset: String },
    FluidField,
    Terrain,
    VoronoiField,
    CellularAutomata,
    WaveformRibbon,
    // New FlowState primitives (render doc §5)
    GlassRain,
    VolumetricLight { sources: Vec<LightSource> },
}

/// One-time GPU resources a module needs to build its pipeline. Passed to a
/// module's constructor; modules are created already-initialised so they never
/// exist in a half-built state.
pub struct ModuleInit<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    /// The format of the offscreen target this module renders into (RGBA16F).
    pub target_format: wgpu::TextureFormat,
}

/// Per-frame inputs for a module's draw. Everything time- or noise-related is
/// supplied by the host so the per-frame path stays reproducible (no wall clock
/// or RNG inside a module — Galene invariant / AV-006).
pub struct FrameCtx<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    /// This layer's target size (already scaled by `resolution_scale`).
    pub resolution: (u32, u32),
    /// Host clock, for animation.
    pub time_secs: f32,
    /// Host-seeded value for grain/flicker.
    pub seed: u32,
    /// Resolved parameters from `resolve_layer_params`.
    pub params: &'a ResolvedParams,
    /// The composited backdrop to sample — `Some` only for refraction modules
    /// (GlassRain, render doc §5.1); `None` for ordinary layers.
    pub backdrop: Option<&'a wgpu::TextureView>,
    /// A strongly-blurred copy of the backdrop (frosted glass), supplied
    /// alongside `backdrop` for refraction modules.
    pub backdrop_blur: Option<&'a wgpu::TextureView>,
}

/// The interface every visual module implements. A module is constructed
/// already-initialised (via its own `new(&ModuleInit, …)`), then drawn each
/// frame into a target the compositor owns.
pub trait VisualModule {
    fn name(&self) -> &str;

    /// Draw this layer into `target` for the current frame, recording into
    /// `encoder`. The module owns its pipeline; the compositor owns the target.
    fn render(
        &mut self,
        ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    );

    /// Whether this module samples the composited backdrop (refraction). Only
    /// GlassRain overrides this to `true` (render doc §5.1).
    fn reads_backdrop(&self) -> bool {
        false
    }
}

/// Build the runtime module for a scene layer's [`ModuleSpec`].
///
/// `ShaderCanvas` is implemented; the others render as a tinted
/// [`PlaceholderModule`] until their real modules land (render doc §5, §11
/// steps 5–6). The tints are rough scene-role hints (dark architecture, warm
/// light, cool glass) so a composited scene is legible meanwhile.
pub fn build_module(spec: &ModuleSpec, init: &ModuleInit) -> Box<dyn VisualModule> {
    match spec {
        ModuleSpec::ShaderCanvas { shader } => {
            Box::new(ShaderCanvasModule::new(init, shader))
        }
        ModuleSpec::GeometricField { preset } => {
            Box::new(GeometricFieldModule::new(init, preset))
        }
        ModuleSpec::ParticleSystem { .. } => {
            Box::new(PlaceholderModule::new(init, "ParticleSystem", [0.85, 0.85, 0.95]))
        }
        ModuleSpec::FluidField => {
            Box::new(PlaceholderModule::new(init, "FluidField", [0.30, 0.30, 0.40]))
        }
        ModuleSpec::Terrain => {
            Box::new(PlaceholderModule::new(init, "Terrain", [0.20, 0.22, 0.24]))
        }
        ModuleSpec::VoronoiField => {
            Box::new(PlaceholderModule::new(init, "VoronoiField", [0.40, 0.30, 0.20]))
        }
        ModuleSpec::CellularAutomata => {
            Box::new(PlaceholderModule::new(init, "CellularAutomata", [0.30, 0.30, 0.30]))
        }
        ModuleSpec::WaveformRibbon => {
            Box::new(PlaceholderModule::new(init, "WaveformRibbon", [0.40, 0.50, 0.60]))
        }
        ModuleSpec::GlassRain => Box::new(GlassRainModule::new(init)),
        ModuleSpec::VolumetricLight { sources } => {
            Box::new(VolumetricLightModule::new(init, sources))
        }
    }
}

/// A fullscreen pass with a single fragment-stage uniform buffer at binding 0 —
/// the common shape of most modules. Reduces per-module pipeline boilerplate.
pub(crate) struct FullscreenUniform {
    pub pipeline: wgpu::RenderPipeline,
    pub uniform: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

/// Build a [`FullscreenUniform`]: `FULLSCREEN_VS` + `fragment_wgsl`, one uniform
/// buffer of `uniform_size` bytes, targeting `init.target_format` (no blend —
/// the compositor applies the layer's blend mode).
pub(crate) fn fullscreen_uniform(
    init: &ModuleInit,
    label: &str,
    fragment_wgsl: &str,
    uniform_size: u64,
) -> FullscreenUniform {
    let device = init.device;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(format!("{FULLSCREEN_VS}\n{fragment_wgsl}").into()),
    });
    let uniform = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: uniform_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform.as_entire_binding(),
        }],
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
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
    FullscreenUniform {
        pipeline,
        uniform,
        bind_group,
    }
}

/// Record a fullscreen draw of `fu` into `target` (clears to transparent first).
pub(crate) fn draw_fullscreen(
    encoder: &mut wgpu::CommandEncoder,
    fu: &FullscreenUniform,
    target: &wgpu::TextureView,
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("module.pass"),
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
    pass.set_pipeline(&fu.pipeline);
    pass.set_bind_group(0, &fu.bind_group, &[]);
    pass.draw(0..3, 0..1);
}

/// A fullscreen-triangle vertex shader shared by fullscreen modules and the
/// compositor: three vertices covering the screen, with 0..1 UVs (y down).
/// Public so external/example modules can build their own fullscreen passes.
pub const FULLSCREEN_VS: &str = r#"
struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> VsOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    let xy = corners[vi];
    var out: VsOut;
    out.pos = vec4<f32>(xy, 0.0, 1.0);
    out.uv = vec2<f32>((xy.x + 1.0) * 0.5, (1.0 - xy.y) * 0.5);
    return out;
}
"#;
