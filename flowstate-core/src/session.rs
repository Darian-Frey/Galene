//! The focus-session engine — session types, intervals, and run state.
//!
//! See `docs/FlowState.md` §9. Transitions between work and break are
//! environmental events, not alarms; the timing lives in `TransitionConfig`.

use serde::{Deserialize, Serialize};

use crate::richness::WorkBreakState;

/// The structure of a focus session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    /// Classic Pomodoro: fixed work/break intervals with a long break.
    Pomodoro {
        work_min: f32,
        break_min: f32,
        long_break_min: f32,
        intervals_until_long: u32,
    },
    /// One long work block with a single break at the midpoint.
    DeepWork { total_min: f32 },
    /// No timer structure; duration tracked but not constrained.
    FreeFlow,
    /// User-defined, saved under a name.
    Custom {
        name: String,
        work_min: f32,
        break_min: f32,
    },
}

impl Default for SessionType {
    /// The product default (FlowState §9.1).
    fn default() -> Self {
        SessionType::Pomodoro {
            work_min: 25.0,
            break_min: 5.0,
            long_break_min: 20.0,
            intervals_until_long: 4,
        }
    }
}

/// A live focus session.
#[derive(Debug, Clone)]
pub struct FocusSession {
    pub session_type: SessionType,
    pub environment_id: crate::environment::EnvironmentId,
    /// Optional work intention entered at setup ("finishing chapter 3").
    pub intention: Option<String>,
    pub state: WorkBreakState,
    /// Whole intervals completed so far.
    pub intervals_completed: u32,
    /// Seconds elapsed in the current interval.
    pub elapsed_secs: f32,
    pub paused: bool,
}

impl FocusSession {
    pub fn new(
        session_type: SessionType,
        environment_id: crate::environment::EnvironmentId,
        intention: Option<String>,
    ) -> Self {
        Self {
            session_type,
            environment_id,
            intention,
            state: WorkBreakState::Work,
            intervals_completed: 0,
            elapsed_secs: 0.0,
            paused: false,
        }
    }

    // TODO(phase-2): interval ticking, work↔break transitions driven by
    // TransitionConfig, interruption logging, and end-of-session recording.
}
