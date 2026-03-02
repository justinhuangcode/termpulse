//! Silent backend — no output.
//!
//! Used when output is not a TTY (pipes, files, CI environments).
//! All operations are no-ops.

use super::Backend;
use crate::ProgressState;

/// Silent backend that produces no output.
///
/// This is the "graceful degradation" endpoint — code that uses termpulse
/// works correctly even when progress cannot be displayed.
#[derive(Debug, Clone, Copy, Default)]
pub struct SilentBackend;

impl Backend for SilentBackend {
    fn emit(&mut self, _state: ProgressState, _percent: Option<u8>, _label: &str) {
        // Intentionally empty
    }

    fn clear(&mut self) {
        // Intentionally empty
    }

    fn name(&self) -> &'static str {
        "silent"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_does_nothing() {
        let mut backend = SilentBackend;
        // These should not panic or produce any side effects
        backend.emit(ProgressState::Normal, Some(50), "test");
        backend.clear();
        assert_eq!(backend.name(), "silent");
    }
}
