//! Scene definition loading — an environment is declarative data, not code
//! (render doc §7). The renderer is general; the twelve environments are twelve
//! RON files under `environments/`.

use serde::{Deserialize, Serialize};

use crate::layer::Layer;
use crate::post::{ColourGrade, PostChain};

/// A complete renderable scene: the layer stack plus its colour grade and post
/// chain. Loaded from an `environments/*.ron` file.
///
/// Note: this is the *rendering* view of an environment. The product-level
/// `flowstate_core::Environment` (work/break states, audio config, session
/// metadata) references a scene by name via `VisualConfig::scene`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Length of one evolution cycle, in minutes.
    pub cycle_minutes: f32,
    /// Per-environment colour grade applied in the post chain.
    pub grade: ColourGrade,
    /// Back-to-front layer stack.
    pub layers: Vec<Layer>,
    pub post: PostChain,
    /// Nyx ambient patch file name (from `flowstate-audio`).
    pub audio_patch: String,
}

impl Scene {
    /// Parse a scene from a RON document.
    pub fn from_ron(src: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped Rainy Library scene must parse against the current types.
    #[test]
    fn rainy_library_parses() {
        let src = include_str!("../../environments/rainy_library.ron");
        let scene = Scene::from_ron(src).expect("rainy_library.ron should parse");
        assert_eq!(scene.id, "rainy_library");
        assert_eq!(scene.layers.len(), 7);
        assert_eq!(scene.audio_patch, "interior_rain.nyx");
    }
}
