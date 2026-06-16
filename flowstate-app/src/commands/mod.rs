//! Command handlers exposed to the frontend.
//!
//! These become Tauri `#[command]` functions once the shell is wired; for now
//! they are plain stubs that document the surface area (FlowState §13.2).

pub mod analytics;
pub mod environment;
pub mod session;
