//! Per-parameter evolution envelope functions (render doc §7).
//!
//! Each maps a cycle position (0–1) to an additive offset, letting every layer
//! parameter drift independently so the scene never returns to exactly the same
//! state within a session.

use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Evolution {
    /// One slow rise-and-fall over the whole cycle, offset by `phase`.
    Drift { amount: f32, phase: f32 },
    /// Sinusoidal variation repeating `1.0 / period_frac` times per cycle.
    Sine { amount: f32, period_frac: f32 },
}

impl Evolution {
    /// Additive offset for the given cycle position (0–1).
    pub fn offset(&self, cycle_position: f32) -> f32 {
        match *self {
            Evolution::Drift { amount, phase } => {
                amount * (cycle_position * 2.0 * PI + phase).sin()
            }
            Evolution::Sine {
                amount,
                period_frac,
            } => {
                let period = if period_frac <= 0.0 { 1.0 } else { period_frac };
                amount * (cycle_position / period * 2.0 * PI).sin()
            }
        }
    }
}

// TODO(phase-1): discrete evolution events (meteors, thunder, …) with
// EventRecurrence, reading from `flowstate_core::EvolutionEvent`.
