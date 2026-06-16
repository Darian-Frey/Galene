//! VolumetricLight — soft additive bloom pools and optional light shafts for
//! lamps, fires, station lights, and geothermal glow (render doc §5.2).

use serde::{Deserialize, Serialize};

use super::VisualModule;

/// A single placeable light source within a VolumetricLight layer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightSource {
    /// Screen-space position, 0–1 on each axis.
    pub pos: (f32, f32),
    /// HDR colour; intensity may exceed 1.0 so bloom catches it.
    pub colour: (f32, f32, f32),
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumetricLight {
    pub sources: Vec<LightSource>,
    pub falloff: f32,
    /// 0 = pool only, >0 = directional god rays.
    pub shaft_strength: f32,
    /// 0–1 flicker amount.
    pub flicker: f32,
}

impl VisualModule for VolumetricLight {
    fn name(&self) -> &str {
        "VolumetricLight"
    }
}

// TODO(phase-0): WGSL (shaders/volumetric_light.wgsl) — additive radial falloff
// per source, optional radial-blur god rays. Composited additively.
