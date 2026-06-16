//! Environment definitions — a complete sensory-world description.
//!
//! Mirrors the structs in `docs/FlowState.md` §6. An environment pairs a subdued
//! `work_state` with a richer `break_state`, plus an evolution system that keeps
//! the world slowly changing so a multi-hour session never feels looped.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Stable identifier for an environment (e.g. `"rainy_library"`).
pub type EnvironmentId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: EnvironmentId,
    pub name: String,
    /// Short sensory description shown in the library.
    pub description: String,
    /// e.g. `["space", "warm", "industrial", "nature"]`.
    pub tags: Vec<String>,
    pub visual: VisualConfig,
    pub audio: AudioConfig,
    /// Shared sensory vocabulary with DreamForge / StoryEngine.
    pub sensory: SensoryProfile,
    pub evolution: EvolutionConfig,
    /// Subdued configuration used during work intervals.
    pub work_state: EnvironmentState,
    /// Rich configuration used during breaks.
    pub break_state: EnvironmentState,
    pub transition: TransitionConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// 0.0–1.0 base richness for this state.
    pub richness: f32,
    /// Module-specific overrides.
    pub visual_params: HashMap<String, f32>,
    /// Patch-specific overrides.
    pub audio_params: HashMap<String, f32>,
    /// How fast the environment evolves in this state.
    pub evolution_rate: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Duration of one environmental cycle.
    pub cycle_minutes: f32,
    /// How much the environment varies over a cycle.
    pub variation_amount: f32,
    /// Discrete scheduled changes (meteors, thunder, …).
    pub events: Vec<EvolutionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    pub name: String,
    /// 0.0–1.0 position in the cycle when this occurs.
    pub cycle_position: f32,
    pub duration_secs: f32,
    pub visual_change: HashMap<String, f32>,
    pub audio_change: HashMap<String, f32>,
    pub recurrence: EventRecurrence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventRecurrence {
    Always,
    /// Triggers with probability `p` each eligible cycle.
    RandomChance(f32),
    OncePerSession,
}

/// Reference to the layer stack / visual configuration. The concrete scene
/// (layers, modules, post chain) lives in `flowstate-visual`; this carries the
/// link plus any environment-level visual parameters. See render doc §7.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualConfig {
    /// Name of the scene definition file under `environments/` (without extension).
    pub scene: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Primary Nyx ambient patch, e.g. `"interior_rain.nyx"`.
    pub primary_patch: String,
    pub secondary_patch: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SensoryProfile {
    /// The atmospheric character described in sensory terms.
    pub description: String,
    pub tags: Vec<String>,
}

/// Timing of the work↔break environmental shifts (render doc §6 / FlowState §9.3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionConfig {
    pub work_to_break_secs: f32,
    pub break_to_work_secs: f32,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            work_to_break_secs: 90.0,
            break_to_work_secs: 60.0,
        }
    }
}
