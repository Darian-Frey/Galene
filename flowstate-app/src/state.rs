//! Shared application state held by the app shell.

use flowstate_core::analytics::AnalyticsStore;

/// Top-level application state. Phase 0+ adds the active session, the loaded
/// environment / driver, and the ambient audio engine.
pub struct AppState {
    pub analytics: AnalyticsStore,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            analytics: AnalyticsStore::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
