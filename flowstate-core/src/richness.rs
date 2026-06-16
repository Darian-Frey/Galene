//! The Richness system — FlowState's most important control.
//!
//! A single master parameter (0.0–1.0) scales visual, audio, and evolution
//! properties together through a non-linear curve. The user's dial value is
//! first resolved against the work/break state so that break is always richer
//! than work. See `docs/FlowState.md` §8 and render doc §6.

use serde::{Deserialize, Serialize};

/// Which half of the work/break cycle the environment is currently in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkBreakState {
    Work,
    Break,
}

/// Smootherstep-style easing, clamped to 0..1. Used to make the richness dial
/// sensitive in the middle range and compressed at the extremes.
pub fn smooth_step(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Resolve the user's dial value against the current state so that the work
/// environment is never as engaging as the break environment.
///
/// - Work  → `0.0..0.5`
/// - Break → `0.5..1.0`
pub fn effective_richness(user_richness: f32, state: WorkBreakState) -> f32 {
    let r = user_richness.clamp(0.0, 1.0);
    match state {
        WorkBreakState::Work => r * 0.5,
        WorkBreakState::Break => 0.5 + r * 0.5,
    }
}

/// The resolved set of properties a richness value maps to. Scene layers and
/// audio patches read these rather than the raw richness scalar.
#[derive(Debug, Clone, Copy)]
pub struct RichnessMapping {
    // Visual
    pub visual_detail: f32,
    pub motion_amount: f32,
    pub particle_density: f32,
    pub effect_intensity: f32,
    pub colour_saturation: f32,
    // Audio
    pub audio_volume: f32,
    pub audio_complexity: f32,
    pub event_frequency: f32,
    pub reverb_presence: f32,
    // Evolution
    pub evolution_speed: f32,
    pub event_probability: f32,
}

impl RichnessMapping {
    /// Non-linear mapping: sensitive in the middle, compressed at the extremes.
    /// Particle density and event frequency are quadratic so they stay sparse
    /// at low richness. Matches `docs/FlowState.md` §8.1.
    pub fn from_richness(r: f32) -> Self {
        let r_curved = smooth_step(r);

        Self {
            visual_detail: 0.1 + r_curved * 0.9,
            motion_amount: 0.05 + r_curved * 0.5,
            particle_density: r_curved * r_curved,
            effect_intensity: 0.2 + r_curved * 0.7,
            colour_saturation: 0.4 + r_curved * 0.6,
            audio_volume: 0.15 + r_curved * 0.7,
            audio_complexity: r_curved,
            event_frequency: r_curved * r_curved,
            reverb_presence: 0.3 + r_curved * 0.6,
            evolution_speed: 0.1 + r_curved * 0.6,
            event_probability: r_curved * 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_is_never_richer_than_break() {
        for i in 0..=10 {
            let r = i as f32 / 10.0;
            assert!(effective_richness(r, WorkBreakState::Work) <= 0.5);
            assert!(effective_richness(r, WorkBreakState::Break) >= 0.5);
        }
    }

    #[test]
    fn particles_stay_sparse_at_low_richness() {
        let low = RichnessMapping::from_richness(0.2);
        let high = RichnessMapping::from_richness(0.9);
        assert!(low.particle_density < high.particle_density);
        assert!(low.particle_density < 0.1);
    }
}
