//! The shared visual-module system.
//!
//! Modules implement a single trait, identical to Synaesthesia's, so a module is
//! written once and used in both products. In Synaesthesia its parameters come
//! from audio analysis; in FlowState from the [`crate::driver::EnvironmentDriver`].
//!
//! Most modules (Particle System, Fluid Field, Geometric Field, Terrain, Voronoi,
//! Cellular Automata, Waveform Ribbon, Shader Canvas) are shared from Synaesthesia
//! (render doc §4). The two new primitives — [`glass_rain`] and
//! [`volumetric_light`] — live here.

pub mod glass_rain;
pub mod volumetric_light;

use serde::{Deserialize, Serialize};

pub use volumetric_light::LightSource;

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

/// The interface every visual module implements. Render wiring (wgpu draw into
/// an offscreen target) is deferred — see [`crate::renderer`].
pub trait VisualModule {
    fn name(&self) -> &str;
}
