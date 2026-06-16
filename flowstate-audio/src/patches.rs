//! The built-in Nyx patch library. Each environment uses one or two patches.
//! See the patch table in `docs/FlowState.md` §12.1.

use serde::{Deserialize, Serialize};

/// Identifies a built-in ambient patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchType {
    DeepSpace,
    InteriorRain,
    WorkshopAmbient,
    StationAmbient,
    StationInterior,
    LargeReverb,
    CoastalOpen,
    GreenhouseRain,
    ArcticWind,
    Maritime,
    CaveAmbient,
    ForestAutumn,
    MillMechanism,
    WaterWheel,
    UrbanRain,
}

impl PatchType {
    /// The `.nyx` file name this patch loads from.
    pub fn file_name(self) -> &'static str {
        match self {
            PatchType::DeepSpace => "deep_space.nyx",
            PatchType::InteriorRain => "interior_rain.nyx",
            PatchType::WorkshopAmbient => "workshop_ambient.nyx",
            PatchType::StationAmbient => "station_ambient.nyx",
            PatchType::StationInterior => "station_interior.nyx",
            PatchType::LargeReverb => "large_reverb.nyx",
            PatchType::CoastalOpen => "coastal_open.nyx",
            PatchType::GreenhouseRain => "greenhouse_rain.nyx",
            PatchType::ArcticWind => "arctic_wind.nyx",
            PatchType::Maritime => "maritime.nyx",
            PatchType::CaveAmbient => "cave_ambient.nyx",
            PatchType::ForestAutumn => "forest_autumn.nyx",
            PatchType::MillMechanism => "mill_mechanism.nyx",
            PatchType::WaterWheel => "water_wheel.nyx",
            PatchType::UrbanRain => "urban_rain.nyx",
        }
    }
}
