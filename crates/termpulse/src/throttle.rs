//! Throttle and deduplication engine for progress updates.
//!
//! Prevents flooding the terminal with too many updates. Enforces a minimum
//! interval between emissions and deduplicates identical consecutive updates.

use crate::ProgressState;
use std::time::{Duration, Instant};

/// Default minimum interval between progress emissions.
pub const DEFAULT_THROTTLE_INTERVAL: Duration = Duration::from_millis(150);

/// Throttle state tracker.
#[derive(Debug)]
pub struct Throttle {
    interval: Duration,
    last_emit: Option<Instant>,
    last_state: Option<ProgressState>,
    last_percent: Option<u8>,
    last_label: String,
}

impl Throttle {
    /// Create a new throttle with the default interval (150ms).
    pub fn new() -> Self {
        Self::with_interval(DEFAULT_THROTTLE_INTERVAL)
    }

    /// Create a new throttle with a custom interval.
    pub fn with_interval(interval: Duration) -> Self {
        Self {
            interval,
            last_emit: None,
            last_state: None,
            last_percent: None,
            last_label: String::new(),
        }
    }

    /// Check if an update should be emitted.
    ///
    /// Returns `true` if the update should be sent to the backend.
    /// An update passes if:
    /// - It's the first update ever
    /// - The state or label changed (always pass immediately)
    /// - Enough time has elapsed since the last emission
    pub fn should_emit(&mut self, state: ProgressState, percent: Option<u8>, label: &str) -> bool {
        let now = Instant::now();

        // First update always passes
        let Some(last_time) = self.last_emit else {
            self.record(now, state, percent, label);
            return true;
        };

        // State or label change passes immediately
        if self.last_state != Some(state) || self.last_label != label {
            self.record(now, state, percent, label);
            return true;
        }

        // Deduplicate identical updates
        if self.last_percent == percent {
            return false;
        }

        // Throttle by time
        if now.duration_since(last_time) >= self.interval {
            self.record(now, state, percent, label);
            return true;
        }

        false
    }

    /// Reset the throttle state.
    pub fn reset(&mut self) {
        self.last_emit = None;
        self.last_state = None;
        self.last_percent = None;
        self.last_label.clear();
    }

    fn record(&mut self, now: Instant, state: ProgressState, percent: Option<u8>, label: &str) {
        self.last_emit = Some(now);
        self.last_state = Some(state);
        self.last_percent = percent;
        self.last_label.clear();
        self.last_label.push_str(label);
    }
}

impl Default for Throttle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_update_passes() {
        let mut t = Throttle::new();
        assert!(t.should_emit(ProgressState::Normal, Some(0), "test"));
    }

    #[test]
    fn duplicate_blocked() {
        let mut t = Throttle::new();
        assert!(t.should_emit(ProgressState::Normal, Some(50), "test"));
        assert!(!t.should_emit(ProgressState::Normal, Some(50), "test"));
    }

    #[test]
    fn state_change_passes() {
        let mut t = Throttle::new();
        assert!(t.should_emit(ProgressState::Normal, Some(50), "test"));
        assert!(t.should_emit(ProgressState::Error, Some(50), "test"));
    }

    #[test]
    fn label_change_passes() {
        let mut t = Throttle::new();
        assert!(t.should_emit(ProgressState::Normal, Some(50), "A"));
        assert!(t.should_emit(ProgressState::Normal, Some(50), "B"));
    }

    #[test]
    fn reset_allows_reemit() {
        let mut t = Throttle::new();
        assert!(t.should_emit(ProgressState::Normal, Some(50), "test"));
        assert!(!t.should_emit(ProgressState::Normal, Some(50), "test"));
        t.reset();
        assert!(t.should_emit(ProgressState::Normal, Some(50), "test"));
    }
}
