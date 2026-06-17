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

pub mod glass_rain;
pub mod placeholder;
pub mod shader_canvas;
pub mod volumetric_light;

use serde::{Deserialize, Serialize};

pub use placeholder::PlaceholderModule;
pub use shader_canvas::ShaderCanvasModule;
pub use volumetric_light::LightSource;

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
        ModuleSpec::GeometricField { .. } => {
            Box::new(PlaceholderModule::new(init, "GeometricField", [0.12, 0.12, 0.16]))
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
        ModuleSpec::GlassRain => {
            Box::new(PlaceholderModule::new(init, "GlassRain", [0.60, 0.70, 0.80]))
        }
        ModuleSpec::VolumetricLight { .. } => {
            Box::new(PlaceholderModule::new(init, "VolumetricLight", [1.0, 0.75, 0.45]))
        }
    }
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
