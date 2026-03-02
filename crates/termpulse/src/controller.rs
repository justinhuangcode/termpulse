//! High-level progress controller.
//!
//! The `Controller` is the primary API for termpulse. It manages backend
//! selection, throttling, and provides a simple interface for progress updates.

use crate::backend::{
    Backend, ascii::AsciiBackend, osc::OscBackend, silent::SilentBackend, tmux::TmuxBackend,
};
use crate::detect::{
    self, DetectOptions, EnvReader, Multiplexer, TerminalCapability, detect_multiplexer,
};
use crate::estimate::Estimator;
use crate::throttle::Throttle;
use termpulse_core::ProgressState;

/// High-level progress controller.
///
/// Automatically detects the best backend and manages throttling,
/// deduplication, and ETA estimation.
///
/// # Example
///
/// ```no_run
/// use termpulse::Controller;
///
/// let mut ctrl = Controller::auto();
/// ctrl.set(25, "Downloading");
/// ctrl.set(50, "Downloading");
/// ctrl.set(100, "Downloading");
/// ctrl.done("Complete");
/// ```
pub struct Controller {
    backend: Box<dyn Backend>,
    throttle: Throttle,
    estimator: Estimator,
    capability: TerminalCapability,
}

impl Controller {
    /// Create a controller with automatic backend detection.
    ///
    /// Detects the terminal capability and selects the best backend:
    /// - OSC 9;4 for supported terminals
    /// - ASCII progress bar for TTY without OSC support
    /// - Silent for non-TTY environments
    pub fn auto() -> Self {
        Self::with_options(&DetectOptions::default())
    }

    /// Create a controller with custom detection options.
    pub fn with_options(opts: &DetectOptions) -> Self {
        let capability = detect::detect(opts);
        let mux = detect_multiplexer(&EnvReader::REAL);

        let backend: Box<dyn Backend> = match capability {
            TerminalCapability::OscProgress => {
                if mux == Multiplexer::Tmux {
                    Box::new(TmuxBackend::stderr())
                } else {
                    Box::new(OscBackend::stderr())
                }
            }
            TerminalCapability::AsciFallback => Box::new(AsciiBackend::stderr()),
            // Silent or any future variant — safe no-op backend.
            _ => Box::new(SilentBackend),
        };

        Self {
            backend,
            throttle: Throttle::new(),
            estimator: Estimator::default(),
            capability,
        }
    }

    /// Create a controller with a specific backend (for testing or custom output).
    pub fn with_backend(backend: Box<dyn Backend>, capability: TerminalCapability) -> Self {
        Self {
            backend,
            throttle: Throttle::new(),
            estimator: Estimator::default(),
            capability,
        }
    }

    /// Set progress to a specific percentage with a label.
    ///
    /// Percentage is clamped to 0-100. Updates are throttled and deduplicated
    /// automatically.
    pub fn set(&mut self, percent: u8, label: &str) {
        let clamped = percent.min(100);
        self.estimator.update(f64::from(clamped));

        if self
            .throttle
            .should_emit(ProgressState::Normal, Some(clamped), label)
        {
            self.backend
                .emit(ProgressState::Normal, Some(clamped), label);
        }
    }

    /// Start indeterminate progress (spinner-like, no percentage).
    pub fn indeterminate(&mut self, label: &str) {
        if self
            .throttle
            .should_emit(ProgressState::Indeterminate, None, label)
        {
            self.backend.emit(ProgressState::Indeterminate, None, label);
        }
    }

    /// Signal successful completion.
    ///
    /// Emits 100% progress, then clears the indicator.
    pub fn done(&mut self, label: &str) {
        self.throttle.reset(); // Bypass throttle for final update
        self.backend.emit(ProgressState::Normal, Some(100), label);
        self.backend.clear();
    }

    /// Signal failure/error.
    ///
    /// Emits error state, then clears the indicator.
    pub fn fail(&mut self, label: &str) {
        self.throttle.reset();
        self.backend.emit(ProgressState::Error, None, label);
        self.backend.clear();
    }

    /// Signal paused state.
    pub fn pause(&mut self, label: &str) {
        if self
            .throttle
            .should_emit(ProgressState::Paused, None, label)
        {
            self.backend.emit(ProgressState::Paused, None, label);
        }
    }

    /// Clear/remove the progress indicator without signaling completion.
    pub fn clear(&mut self) {
        self.throttle.reset();
        self.backend.clear();
    }

    /// Get the detected terminal capability.
    pub fn capability(&self) -> TerminalCapability {
        self.capability
    }

    /// Get the backend name (e.g., "osc", "ascii", "silent").
    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    /// Get estimated time remaining.
    pub fn eta_display(&self) -> String {
        self.estimator.eta_display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Backend;
    use std::sync::{Arc, Mutex};

    type CallLog = Arc<Mutex<Vec<(ProgressState, Option<u8>, String)>>>;

    #[derive(Clone)]
    struct RecordingBackend {
        calls: CallLog,
        cleared: Arc<Mutex<u32>>,
    }

    impl RecordingBackend {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(Vec::new())),
                cleared: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl Backend for RecordingBackend {
        fn emit(&mut self, state: ProgressState, percent: Option<u8>, label: &str) {
            self.calls
                .lock()
                .unwrap()
                .push((state, percent, label.to_string()));
        }

        fn clear(&mut self) {
            *self.cleared.lock().unwrap() += 1;
        }

        fn name(&self) -> &'static str {
            "recording"
        }
    }

    #[test]
    fn set_emits_progress() {
        let rec = RecordingBackend::new();
        let calls = rec.calls.clone();
        let mut ctrl = Controller::with_backend(Box::new(rec), TerminalCapability::OscProgress);
        ctrl.set(50, "Building");

        let log = calls.lock().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(
            log[0],
            (ProgressState::Normal, Some(50), "Building".to_string())
        );
    }

    #[test]
    fn done_emits_100_and_clears() {
        let rec = RecordingBackend::new();
        let calls = rec.calls.clone();
        let cleared = rec.cleared.clone();
        let mut ctrl = Controller::with_backend(Box::new(rec), TerminalCapability::OscProgress);
        ctrl.done("Done");

        let log = calls.lock().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].1, Some(100));
        assert_eq!(*cleared.lock().unwrap(), 1);
    }

    #[test]
    fn fail_emits_error_and_clears() {
        let rec = RecordingBackend::new();
        let calls = rec.calls.clone();
        let cleared = rec.cleared.clone();
        let mut ctrl = Controller::with_backend(Box::new(rec), TerminalCapability::OscProgress);
        ctrl.fail("Error");

        let log = calls.lock().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, ProgressState::Error);
        assert_eq!(*cleared.lock().unwrap(), 1);
    }

    #[test]
    fn percent_clamped_to_100() {
        let rec = RecordingBackend::new();
        let calls = rec.calls.clone();
        let mut ctrl = Controller::with_backend(Box::new(rec), TerminalCapability::OscProgress);
        ctrl.set(200, "Over");

        let log = calls.lock().unwrap();
        assert_eq!(log[0].1, Some(100));
    }

    #[test]
    fn throttle_deduplicates() {
        let rec = RecordingBackend::new();
        let calls = rec.calls.clone();
        let mut ctrl = Controller::with_backend(Box::new(rec), TerminalCapability::OscProgress);
        ctrl.set(50, "A");
        ctrl.set(50, "A"); // duplicate — should be blocked
        ctrl.set(50, "A"); // duplicate — should be blocked

        let log = calls.lock().unwrap();
        assert_eq!(log.len(), 1);
    }
}
