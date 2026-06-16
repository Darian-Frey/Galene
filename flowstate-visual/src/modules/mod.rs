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
pub mod shader_canvas;
pub mod volumetric_light;

use serde::{Deserialize, Serialize};

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

/// A fullscreen-triangle vertex shader shared by fullscreen modules and the
/// compositor: three vertices covering the screen, with 0..1 UVs (y down).
pub(crate) const FULLSCREEN_VS: &str = r#"
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
