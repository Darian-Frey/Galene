//! Slow, organic variation of audio parameters over the evolution cycle.
//!
//! Each parameter drifts on its own phase offset so that even a two-hour session
//! never sounds exactly as it did an hour ago. See `docs/FlowState.md` §12.3.

use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Debug, Clone, Default)]
pub struct AudioEvolution {
    pub base_params: HashMap<String, f32>,
    /// Maximum variation from base, per parameter.
    pub variation: HashMap<String, f32>,
    /// Phase offset per parameter so they drift independently.
    pub phase_offsets: HashMap<String, f32>,
}

impl AudioEvolution {
    /// Resolve the current parameter values for a given cycle position (0–1).
    pub fn current_params(&self, cycle_position: f32) -> HashMap<String, f32> {
        self.base_params
            .iter()
            .map(|(k, base)| {
                let variation = self.variation.get(k).copied().unwrap_or(0.0);
                let phase = self.phase_offsets.get(k).copied().unwrap_or(0.0);
                let drift = variation * (cycle_position * 2.0 * PI + phase).sin();
                (k.clone(), (base + drift).clamp(0.0, 1.0))
            })
            .collect()
    }
}
