//! The EnvironmentDriver — Galene's replacement for Synaesthesia's mapping
//! graph as the source of visual-module parameters (render doc §6, FlowState
//! §13.3).

use flowstate_core::{
    effective_richness, EvolutionState, RichnessMapping, TransitionConfig, WorkBreakState,
};

use crate::layer::{resolve_layer_params, DriverContext, Layer, ResolvedParams};
use crate::scene::Scene;

/// Owns a loaded scene and the live evolution / richness / work-break state, and
/// produces the current parameters for each layer every frame.
pub struct EnvironmentDriver {
    pub scene: Scene,
    pub evolution: EvolutionState,
    /// Raw user richness dial, 0..1.
    pub richness: f32,
    /// The state we are in or transitioning toward.
    pub state: WorkBreakState,
    /// 0.0 in work, 1.0 in break; animated across the transition by `advance`.
    pub state_blend: f32,
    /// Transition timing (90s work→break, 60s break→work by default).
    pub transition: TransitionConfig,
}

impl EnvironmentDriver {
    pub fn new(scene: Scene) -> Self {
        let evolution = EvolutionState::new(scene.cycle_minutes);
        Self {
            scene,
            evolution,
            richness: 0.45, // FlowState §8.3 default
            state: WorkBreakState::Work,
            state_blend: 0.0,
            transition: TransitionConfig::default(),
        }
    }

    /// Begin a transition toward `state`. `advance` animates `state_blend`
    /// toward the new target over the configured transition duration.
    pub fn set_state(&mut self, state: WorkBreakState) {
        self.state = state;
    }

    /// Advance the driver by `dt_secs`: tick the evolution cycle and move
    /// `state_blend` toward its target (1.0 for break, 0.0 for work).
    pub fn advance(&mut self, dt_secs: f32) {
        let r = effective_richness(self.richness, self.state);
        let evolution_speed = RichnessMapping::from_richness(r).evolution_speed;
        self.evolution.advance(dt_secs, evolution_speed);

        let (target, dur) = match self.state {
            WorkBreakState::Work => (0.0_f32, self.transition.break_to_work_secs),
            WorkBreakState::Break => (1.0_f32, self.transition.work_to_break_secs),
        };
        if dur > 0.0 {
            let step = dt_secs / dur;
            if self.state_blend < target {
                self.state_blend = (self.state_blend + step).min(target);
            } else {
                self.state_blend = (self.state_blend - step).max(target);
            }
        } else {
            self.state_blend = target;
        }
    }

    fn context(&self) -> DriverContext {
        DriverContext {
            user_richness: self.richness,
            state: self.state,
            cycle_position: self.evolution.cycle_position,
            state_blend: self.state_blend,
        }
    }

    /// Resolve current parameters for one layer.
    pub fn resolve(&self, layer: &Layer) -> ResolvedParams {
        resolve_layer_params(layer, &self.context())
    }

    /// Resolve parameters for every layer in the scene, in stack order.
    pub fn resolve_all(&self) -> Vec<ResolvedParams> {
        let ctx = self.context();
        self.scene
            .layers
            .iter()
            .map(|layer| resolve_layer_params(layer, &ctx))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn driver() -> EnvironmentDriver {
        let src = include_str!("../../environments/rainy_library.ron");
        EnvironmentDriver::new(Scene::from_ron(src).unwrap())
    }

    #[test]
    fn transition_animates_state_blend_toward_break() {
        let mut d = driver();
        assert_eq!(d.state_blend, 0.0);
        d.set_state(WorkBreakState::Break);
        // 90s work→break transition; advance halfway.
        d.advance(45.0);
        assert!((d.state_blend - 0.5).abs() < 1e-4, "blend = {}", d.state_blend);
        d.advance(45.0);
        assert!((d.state_blend - 1.0).abs() < 1e-4);
        // Further advance does not overshoot.
        d.advance(10.0);
        assert_eq!(d.state_blend, 1.0);
    }

    #[test]
    fn transition_back_to_work_uses_shorter_duration() {
        let mut d = driver();
        d.state = WorkBreakState::Break;
        d.state_blend = 1.0;
        d.set_state(WorkBreakState::Work);
        d.advance(60.0); // full break→work duration
        assert!((d.state_blend - 0.0).abs() < 1e-4);
    }

    #[test]
    fn evolution_advances_and_wraps() {
        let mut d = driver();
        d.richness = 1.0;
        d.state = WorkBreakState::Break;
        let start = d.evolution.cycle_position;
        d.advance(1.0);
        assert!(d.evolution.cycle_position > start);
        assert!((0.0..1.0).contains(&d.evolution.cycle_position));
    }
}
