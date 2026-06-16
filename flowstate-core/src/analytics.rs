//! Session analytics — local-only recording and the insights engine.
//!
//! All tracking is local; no data leaves the device (FlowState §10). Persistence
//! is deferred to Phase 2 (SQLite via `rusqlite`); for now records are held in
//! memory.

use serde::{Deserialize, Serialize};

/// One completed (or abandoned) session, as stored for analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    /// Unix timestamp (seconds) of session start. Stamped by the caller —
    /// `flowstate-core` does not read the wall clock.
    pub started_at: i64,
    pub duration_secs: f32,
    pub environment_id: String,
    pub session_type: String,
    /// Average richness over the session, and its range.
    pub avg_richness: f32,
    pub richness_min: f32,
    pub richness_max: f32,
    pub intention: Option<String>,
    pub intervals_completed: u32,
    pub intervals_total: u32,
    pub interruptions: u32,
    /// 1–5 quality rating, if the user provided one.
    pub quality: Option<u8>,
}

/// In-memory store of session records. Phase 2 swaps the backing for SQLite.
#[derive(Debug, Default)]
pub struct AnalyticsStore {
    records: Vec<SessionRecord>,
}

impl AnalyticsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, record: SessionRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[SessionRecord] {
        &self.records
    }

    // TODO(phase-2): derived metrics (preferred environments, optimal richness
    // range, best focus times, streaks) and the insights engine that surfaces
    // observations after ~20 sessions (FlowState §10.3).
}
