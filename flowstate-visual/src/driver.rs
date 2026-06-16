//! The EnvironmentDriver — FlowState's replacement for Synaesthesia's mapping
//! graph as the source of visual-module parameters (render doc §6, FlowState
//! §13.3).

use flowstate_core::{EvolutionState, WorkBreakState};

use crate::layer::{resolve_layer_params, DriverContext, Layer, ResolvedParams};
use crate::scene::Scene;

/// Owns a loaded scene and the live evolution / richness / work-break state, and
/// produces the current parameters for each layer every frame.
pub struct EnvironmentDriver {
    pub scene: Scene,
    pub evolution: EvolutionState,
    /// Raw user richness dial, 0..1.
    pub richness: f32,
    pub state: WorkBreakState,
    /// 0.0 in work, 1.0 in break; moved across the transition.
    pub state_blend: f32,
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

    // TODO(phase-0): advance() to tick evolution and move state_blend across the
    // work↔break transition using TransitionConfig timing.
}
