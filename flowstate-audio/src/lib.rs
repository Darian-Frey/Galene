//! # flowstate-audio
//!
//! Drives FlowState's ambient soundscapes. The audio engine is parameterised
//! synthesis (Nyx), not recorded audio, so patches never loop or seam. Patch
//! parameters are driven by the resolved richness value plus a slow evolution
//! cycle. See `docs/FlowState.md` §12.

pub mod ambient_engine;
pub mod evolution_audio;
pub mod patches;

pub use ambient_engine::AmbientAudioEngine;
pub use evolution_audio::AudioEvolution;
pub use patches::PatchType;
