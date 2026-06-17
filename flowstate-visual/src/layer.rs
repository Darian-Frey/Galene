//! The layer model and per-frame parameter resolution (render doc §2, §6).

use std::collections::{BTreeSet, HashMap};

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

/// Resolve a layer's current parameters from base + work/break + evolution +
/// richness (render doc §6, FlowState §8.2).
///
/// Resolution per key:
/// 1. **Baseline** — the work/break lerp if the key is authored in either state
///    map, otherwise the `base` value. (Work/break states carry per-key
///    authored richness levels.)
/// 2. **Evolution** — additive slow drift on the layer's own phase offset.
/// 3. **Richness** — a master multiplier so the user's dial visibly affects
///    output (F-003); the mapping field is chosen by parameter name (see
///    [`richness_scale_for`]). Unmatched params are left unscaled.
///
/// This reorders the design-doc pseudocode (which lerps last) so that the dial
/// also scales authored work/break params — see DECISIONS D-010.
pub fn resolve_layer_params(layer: &Layer, ctx: &DriverContext) -> ResolvedParams {
    let r = effective_richness(ctx.user_richness, ctx.state);
    let m = RichnessMapping::from_richness(r);

    // Every parameter key the layer mentions, in any of its maps.
    let mut keys: BTreeSet<&String> = BTreeSet::new();
    keys.extend(layer.base.keys());
    keys.extend(layer.work.keys());
    keys.extend(layer.break_state.keys());
    keys.extend(layer.evolution.keys());

    let mut p = ResolvedParams::with_capacity(keys.len());
    for key in keys {
        // 1. Baseline: authored work/break lerp, else the base value.
        let baseline = if layer.work.contains_key(key) || layer.break_state.contains_key(key) {
            let work = layer
                .work
                .get(key)
                .or_else(|| layer.break_state.get(key))
                .copied()
                .unwrap_or(0.0);
            let brk = layer
                .break_state
                .get(key)
                .or_else(|| layer.work.get(key))
                .copied()
                .unwrap_or(0.0);
            work + (brk - work) * ctx.state_blend
        } else {
            layer.base.get(key).copied().unwrap_or(0.0)
        };

        // 2. Evolution drift (additive, per-parameter phase).
        let drift = layer
            .evolution
            .get(key)
            .map(|e| e.offset(ctx.cycle_position))
            .unwrap_or(0.0);

        // 3. Master richness scaling (multiplicative; matched by name).
        let scale = richness_scale_for(key, &m);

        p.insert(key.clone(), ((baseline + drift) * scale).clamp(0.0, 1.0));
    }
    p
}

/// Choose which [`RichnessMapping`] field scales a parameter, by name. Density-
/// and intensity-like params are scaled; structural params (darkness, warmth,
/// storm visibility, …) are left untouched (factor 1.0). Provisional — see
/// DECISIONS D-010.
fn richness_scale_for(key: &str, m: &RichnessMapping) -> f32 {
    let k = key.to_ascii_lowercase();
    let has = |needles: &[&str]| needles.iter().any(|n| k.contains(n));

    // Precipitation is intensity, not a particle count — it should stay present
    // even at low richness (the Rainy Library is always raining), so it uses the
    // gentler effect curve rather than the quadratic particle curve.
    if has(&["rain", "snow"]) {
        m.effect_intensity
    } else if has(&["density", "particle", "dust", "leaf", "leaves"]) {
        m.particle_density
    } else if has(&["intensity", "glow", "bloom", "flicker", "effect", "brightness"]) {
        m.effect_intensity
    } else if has(&["motion", "speed", "drift", "run_"]) {
        m.motion_amount
    } else if has(&["saturation", "colour", "color", "vibrancy"]) {
        m.colour_saturation
    } else if has(&["detail"]) {
        m.visual_detail
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::ModuleSpec;

    fn ctx(user_richness: f32, state: WorkBreakState, state_blend: f32) -> DriverContext {
        DriverContext {
            user_richness,
            state,
            cycle_position: 0.0,
            state_blend,
        }
    }

    fn layer_with(
        base: &[(&str, f32)],
        work: &[(&str, f32)],
        brk: &[(&str, f32)],
    ) -> Layer {
        let to_map = |kv: &[(&str, f32)]| -> HashMap<String, f32> {
            kv.iter().map(|(k, v)| (k.to_string(), *v)).collect()
        };
        Layer {
            name: "t".into(),
            module: ModuleSpec::GlassRain,
            blend: BlendMode::Normal,
            depth_blur: 0.0,
            resolution_scale: 1.0,
            base: to_map(base),
            evolution: HashMap::new(),
            work: to_map(work),
            break_state: to_map(brk),
        }
    }

    #[test]
    fn work_break_lerp_on_unscaled_param() {
        let l = layer_with(&[], &[("storm_visibility", 0.4)], &[("storm_visibility", 0.8)]);
        let full_break = resolve_layer_params(&l, &ctx(1.0, WorkBreakState::Break, 1.0));
        assert!((full_break["storm_visibility"] - 0.8).abs() < 1e-4);
        let mid = resolve_layer_params(&l, &ctx(1.0, WorkBreakState::Break, 0.5));
        assert!((mid["storm_visibility"] - 0.6).abs() < 1e-4);
    }

    #[test]
    fn richness_dial_scales_density_params() {
        // A particle-count param is crushed toward zero at low richness (quadratic).
        let particles = layer_with(&[("particle_density", 0.9)], &[], &[]);
        let low =
            resolve_layer_params(&particles, &ctx(0.0, WorkBreakState::Work, 0.0))["particle_density"];
        let high =
            resolve_layer_params(&particles, &ctx(1.0, WorkBreakState::Break, 1.0))["particle_density"];
        assert!(low < high);
        assert!(low < 0.05, "particles near-zero at min richness, got {low}");

        // Rain is precipitation intensity: it scales with the dial but stays
        // present even at minimum richness (gentler curve, never crushed to ~0).
        let rain = layer_with(&[("rain_density", 0.9)], &[], &[]);
        let rlow = resolve_layer_params(&rain, &ctx(0.0, WorkBreakState::Work, 0.0))["rain_density"];
        let rhigh = resolve_layer_params(&rain, &ctx(1.0, WorkBreakState::Break, 1.0))["rain_density"];
        assert!(rlow < rhigh);
        assert!(rlow > 0.1, "rain should stay present at low richness, got {rlow}");
    }

    #[test]
    fn structural_params_are_not_richness_scaled() {
        let l = layer_with(&[("darkness", 0.6)], &[], &[]);
        let p = resolve_layer_params(&l, &ctx(0.0, WorkBreakState::Work, 0.0));
        assert!((p["darkness"] - 0.6).abs() < 1e-4);
    }
}
