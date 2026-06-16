//! The post-processing chain (render doc §5.3).
//!
//! Order: DOF far-haze → bloom → colour grade → vignette → film grain → tone-map.
//! The grade and grain do most of the painterly work; a clean digital image
//! looks wrong for this product.

pub mod bloom;
pub mod grade;
pub mod grain;
pub mod vignette;

use serde::{Deserialize, Serialize};

pub use grade::ColourGrade;

/// Per-environment post-chain parameters, as stored in the scene definition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PostChain {
    pub bloom_threshold: f32,
    pub bloom_intensity: f32,
    pub vignette_amount: f32,
    pub vignette_softness: f32,
    /// Subtle — 0.02–0.06 typical.
    pub grain_amount: f32,
    /// Optional global far-haze depth-of-field.
    #[serde(default)]
    pub dof_far_haze: f32,
}
