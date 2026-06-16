//! The evolution cycle — tracks position within an environment's cycle so that
//! slow, organic variation can be derived from it. The per-parameter envelope
//! functions (Drift, Sine) that read this position live in
//! `flowstate-visual::evolution_visual` and `flowstate-audio::evolution_audio`.

/// Current position within an environment's evolution cycle.
#[derive(Debug, Clone, Default)]
pub struct EvolutionState {
    /// 0.0–1.0 position in the cycle.
    pub cycle_position: f32,
    /// Length of one full cycle, in minutes (from `EvolutionConfig`).
    pub cycle_minutes: f32,
}

impl EvolutionState {
    pub fn new(cycle_minutes: f32) -> Self {
        Self {
            cycle_position: 0.0,
            cycle_minutes,
        }
    }

    /// Advance the cycle by `dt_secs`, scaled by `evolution_speed` (from the
    /// resolved richness mapping). Wraps around at 1.0.
    pub fn advance(&mut self, dt_secs: f32, evolution_speed: f32) {
        if self.cycle_minutes <= 0.0 {
            return;
        }
        let cycle_secs = self.cycle_minutes * 60.0;
        let delta = (dt_secs * evolution_speed) / cycle_secs;
        self.cycle_position = (self.cycle_position + delta).rem_euclid(1.0);
    }
}
