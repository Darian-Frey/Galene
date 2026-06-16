//! GlassRain — screen-space refraction with running droplet trails on an implied
//! pane of glass (render doc §5.1). The most reused new primitive: Library,
//! Workshop, Greenhouse, Chart Room, Midnight City.
//!
//! Unlike other layers it reads the already-composited back buffer and offsets
//! sample UVs for refraction, so it composites with [`crate::layer::BlendMode::Refraction`].

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlassRain {
    pub rain_density: f32,
    pub droplet_size: f32,
    pub trail_length: f32,
    pub run_speed: f32,
    pub refraction_strength: f32,
    /// Frosted-haze amount, 0–1.
    pub glass_fog: f32,
}

impl Default for GlassRain {
    fn default() -> Self {
        Self {
            rain_density: 0.5,
            droplet_size: 0.5,
            trail_length: 0.5,
            run_speed: 0.5,
            refraction_strength: 0.3,
            glass_fog: 0.2,
        }
    }
}

// TODO(phase-0): implement `VisualModule` (render into the layer target reading
// the backdrop, `reads_backdrop` → true) once the compositor's back-buffer path
// lands. The struct above holds the parameters until then.

// TODO(phase-0): WGSL fragment shader (shaders/glass_rain.wgsl) — procedural
// droplet field sampling the back buffer with a refraction offset and vertical
// trails. See render doc §5.1.
