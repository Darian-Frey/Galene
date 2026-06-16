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
    /// One long work block (`total_min` of work) with one break of `break_min`
    /// at the midpoint.
    DeepWork { total_min: f32, break_min: f32 },
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

/// What a [`FocusSession::tick`] produced this step. The app uses these to
/// drive the work↔break environmental transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionEvent {
    /// No interval boundary crossed this tick.
    None,
    /// A work interval ended; the session entered a break (`long` = long break).
    EnteredBreak { long: bool },
    /// A break ended; the session returned to work.
    EnteredWork,
    /// The session reached its end (Deep Work after the second half).
    Completed,
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
    /// True once the session has reached a definite end (Deep Work).
    pub completed: bool,
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
            completed: false,
        }
    }

    /// Toggle the pause state. A paused session ignores `tick`.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Advance the session by `dt_secs`. Returns any interval boundary crossed.
    /// A paused or completed session does not advance. Free Flow is untimed (the
    /// elapsed time accrues but no boundary fires).
    pub fn tick(&mut self, dt_secs: f32) -> SessionEvent {
        if self.paused || self.completed {
            return SessionEvent::None;
        }
        self.elapsed_secs += dt_secs;

        match self.session_type {
            SessionType::Pomodoro {
                work_min,
                break_min,
                long_break_min,
                intervals_until_long,
            } => self.tick_intervals(work_min, break_min, long_break_min, intervals_until_long),
            // Custom has no long break: pass an unreachable threshold.
            SessionType::Custom {
                work_min,
                break_min,
                ..
            } => self.tick_intervals(work_min, break_min, break_min, 0),
            SessionType::FreeFlow => SessionEvent::None,
            SessionType::DeepWork {
                total_min,
                break_min,
            } => self.tick_deep_work(total_min, break_min),
        }
    }

    /// Shared work/break interval logic. `intervals_until_long == 0` disables the
    /// long break (used by Custom sessions).
    fn tick_intervals(
        &mut self,
        work_min: f32,
        break_min: f32,
        long_break_min: f32,
        intervals_until_long: u32,
    ) -> SessionEvent {
        let is_long = |completed: u32| {
            intervals_until_long != 0 && completed != 0 && completed.is_multiple_of(intervals_until_long)
        };

        match self.state {
            WorkBreakState::Work => {
                if self.elapsed_secs >= work_min * 60.0 {
                    self.intervals_completed += 1;
                    self.elapsed_secs = 0.0;
                    self.state = WorkBreakState::Break;
                    SessionEvent::EnteredBreak {
                        long: is_long(self.intervals_completed),
                    }
                } else {
                    SessionEvent::None
                }
            }
            WorkBreakState::Break => {
                let break_len = if is_long(self.intervals_completed) {
                    long_break_min
                } else {
                    break_min
                };
                if self.elapsed_secs >= break_len * 60.0 {
                    self.elapsed_secs = 0.0;
                    self.state = WorkBreakState::Work;
                    SessionEvent::EnteredWork
                } else {
                    SessionEvent::None
                }
            }
        }
    }

    /// Deep Work: work the first half of `total_min`, take one `break_min`
    /// break at the midpoint, work the second half, then complete. The break
    /// is additional to `total_min` (which is work time only).
    fn tick_deep_work(&mut self, total_min: f32, break_min: f32) -> SessionEvent {
        let half_secs = total_min * 60.0 / 2.0;
        match self.state {
            WorkBreakState::Work => {
                if self.elapsed_secs >= half_secs {
                    self.intervals_completed += 1;
                    self.elapsed_secs = 0.0;
                    if self.intervals_completed == 1 {
                        // midpoint reached → take the break
                        self.state = WorkBreakState::Break;
                        SessionEvent::EnteredBreak { long: false }
                    } else {
                        // second half done → session complete
                        self.completed = true;
                        SessionEvent::Completed
                    }
                } else {
                    SessionEvent::None
                }
            }
            WorkBreakState::Break => {
                if self.elapsed_secs >= break_min * 60.0 {
                    self.elapsed_secs = 0.0;
                    self.state = WorkBreakState::Work;
                    SessionEvent::EnteredWork
                } else {
                    SessionEvent::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pomodoro_1_1_2_every_2() -> FocusSession {
        FocusSession::new(
            SessionType::Pomodoro {
                work_min: 1.0,
                break_min: 1.0,
                long_break_min: 2.0,
                intervals_until_long: 2,
            },
            "rainy_library".into(),
            None,
        )
    }

    #[test]
    fn work_interval_ends_into_a_short_break() {
        let mut s = pomodoro_1_1_2_every_2();
        assert_eq!(s.tick(59.0), SessionEvent::None);
        assert_eq!(s.tick(1.0), SessionEvent::EnteredBreak { long: false });
        assert_eq!(s.state, WorkBreakState::Break);
        assert_eq!(s.intervals_completed, 1);
    }

    #[test]
    fn every_nth_break_is_long() {
        let mut s = pomodoro_1_1_2_every_2();
        // Interval 1 → short break → back to work.
        assert_eq!(s.tick(60.0), SessionEvent::EnteredBreak { long: false });
        assert_eq!(s.tick(60.0), SessionEvent::EnteredWork);
        // Interval 2 → long break (2 % 2 == 0).
        assert_eq!(s.tick(60.0), SessionEvent::EnteredBreak { long: true });
        // Long break is 2 minutes: not over at 60s, over at 120s.
        assert_eq!(s.tick(60.0), SessionEvent::None);
        assert_eq!(s.tick(60.0), SessionEvent::EnteredWork);
    }

    #[test]
    fn paused_session_does_not_advance() {
        let mut s = pomodoro_1_1_2_every_2();
        s.toggle_pause();
        assert_eq!(s.tick(120.0), SessionEvent::None);
        assert_eq!(s.elapsed_secs, 0.0);
        assert_eq!(s.state, WorkBreakState::Work);
    }

    #[test]
    fn free_flow_accrues_time_without_boundaries() {
        let mut s = FocusSession::new(SessionType::FreeFlow, "rainy_library".into(), None);
        assert_eq!(s.tick(3600.0), SessionEvent::None);
        assert_eq!(s.elapsed_secs, 3600.0);
    }

    #[test]
    fn deep_work_breaks_at_midpoint_then_completes() {
        // 4-minute block, 1-minute midpoint break → halves of 2 minutes each.
        let mut s = FocusSession::new(
            SessionType::DeepWork {
                total_min: 4.0,
                break_min: 1.0,
            },
            "rainy_library".into(),
            None,
        );
        // First half (2 min) → midpoint break.
        assert_eq!(s.tick(119.0), SessionEvent::None);
        assert_eq!(s.tick(1.0), SessionEvent::EnteredBreak { long: false });
        assert_eq!(s.state, WorkBreakState::Break);
        assert_eq!(s.intervals_completed, 1);
        // Break (1 min) → second half.
        assert_eq!(s.tick(60.0), SessionEvent::EnteredWork);
        assert_eq!(s.state, WorkBreakState::Work);
        // Second half (2 min) → complete.
        assert_eq!(s.tick(119.0), SessionEvent::None);
        assert_eq!(s.tick(1.0), SessionEvent::Completed);
        assert!(s.completed);
        assert_eq!(s.intervals_completed, 2);
        // A completed session does not advance further.
        assert_eq!(s.tick(600.0), SessionEvent::None);
    }
}
