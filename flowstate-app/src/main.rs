//! FlowState desktop application entry point.
//!
//! This is a placeholder binary that wires the workspace crates together. The
//! Tauri shell, window, and TypeScript frontend (FlowState §13.2) arrive with
//! the Phase 0 app scaffold; the eventual command handlers are stubbed under
//! [`commands`].

mod commands;
mod state;

use state::AppState;

fn main() {
    let app = AppState::new();
    println!("FlowState {} — core scaffold loaded.", env!("CARGO_PKG_VERSION"));
    println!("Sessions recorded so far: {}", app.analytics.records().len());
    println!("(Tauri shell not yet wired — see flowstate-app/Cargo.toml.)");
}
