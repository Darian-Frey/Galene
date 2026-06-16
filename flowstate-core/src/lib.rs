//! # flowstate-core
//!
//! The non-rendering heart of FlowState: environment definitions, the Richness
//! system, the evolution cycle, the focus-session engine, and session analytics.
//!
//! See `docs/FlowState.md` §6–§10 for the product design these types implement.

pub mod analytics;
pub mod environment;
pub mod evolution;
pub mod richness;
pub mod serialiser;
pub mod session;

pub use environment::{
    AudioConfig, Environment, EnvironmentId, EnvironmentState, EvolutionConfig, EvolutionEvent,
    EventRecurrence, SensoryProfile, TransitionConfig, VisualConfig,
};
pub use evolution::EvolutionState;
pub use richness::{effective_richness, smooth_step, RichnessMapping, WorkBreakState};
pub use session::{FocusSession, SessionEvent, SessionType};
