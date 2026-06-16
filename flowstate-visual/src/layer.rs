//! The layer model and per-frame parameter resolution (render doc §2, §6).

use std::collections::HashMap;

use flowstate_core::{effective_richness, RichnessMapping, WorkBreakState};
use serde::{Deserialize, Serialize};

use crate::evolution_visual::Evolution;
use crate::modules::ModuleSpec;

/// How a layer is composited onto the accumulation target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    /// Standard alpha — scene layers.
    Normal,
    /// `one, one` — light and dust layers brighten what is behind them.
    Additive,
    /// Reads the back buffer and offsets sample UVs — GlassRain (render doc §5.1).
    Refraction,
}

/// One layer in an environment's back-to-front stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    /// The module that draws this layer.
    pub module: ModuleSpec,
    pub blend: BlendMode,
    /// 0.0 = sharp foreground, 1.0 = heavily depth-blurred far layer.
    pub depth_blur: f32,
    /// Render-target scale; blurred far layers can render at half/quarter res.
    #[serde(default = "default_resolution_scale")]
    pub resolution_scale: f32,
    /// Base parameter values from the scene definition.
    #[serde(default)]
    pub base: HashMap<String, f32>,
    /// Per-parameter evolution envelopes (slow drift over the cycle).
    #[serde(default)]
    pub evolution: HashMap<String, Evolution>,
    /// Work-state parameter overrides.
    #[serde(default)]
    pub work: HashMap<String, f32>,
    /// Break-state parameter overrides.
    #[serde(default, rename = "break")]
    pub break_state: HashMap<String, f32>,
}

fn default_resolution_scale() -> f32 {
    1.0
}

/// Per-frame inputs the driver supplies when resolving layer parameters.
#[derive(Debug, Clone, Copy)]
pub struct DriverContext {
    /// Raw user richness dial value, 0..1.
    pub user_richness: f32,
    pub state: WorkBreakState,
    /// 0..1 position within the evolution cycle.
    pub cycle_position: f32,
    /// 0.0 in work, 1.0 in break; interpolates across the transition.
    pub state_blend: f32,
}

/// The resolved parameter set a module renders from this frame.
pub type ResolvedParams = HashMap<String, f32>;

/// Resolve a layer's current parameters from base + evolution + richness +
/// work/break state. Mirrors the resolution order in render doc §6.
pub fn resolve_layer_params(layer: &Layer, ctx: &DriverContext) -> ResolvedParams {
    let r = effective_richness(ctx.user_richness, ctx.state);
    let m = RichnessMapping::from_richness(r);

    let mut p = layer.base.clone();

    // 1. Slow evolution drift, per-parameter phase offsets.
    for (key, env) in &layer.evolution {
        let entry = p.entry(key.clone()).or_insert(0.0);
        *entry += env.offset(ctx.cycle_position);
    }

    // 2. Scale by the richness mapping. Which curve applies to which key is
    //    module-specific; for now intensity-like params scale by effect_intensity.
    //    TODO(phase-0): per-key richness scaling table.
    let _ = m;

    // 3. Lerp from work toward break by state_blend.
    for (key, break_val) in &layer.break_state {
        let work_val = layer.work.get(key).copied().unwrap_or(*break_val);
        let blended = work_val + (break_val - work_val) * ctx.state_blend;
        p.insert(key.clone(), blended);
    }

    p
}
