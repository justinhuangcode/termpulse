//! Output backends for progress indicators.
//!
//! Termpulse uses a three-tier fallback strategy:
//!
//! 1. **OSC** — native terminal progress via OSC 9;4 sequences
//! 2. **ASCII** — visual `[=====>   ] 50%` progress bar on stderr
//! 3. **Silent** — no output (for pipes, files, CI)

pub mod ascii;
pub mod osc;
pub mod silent;
pub mod tmux;

use crate::ProgressState;

/// Trait for progress output backends.
///
/// All backends implement this trait, allowing the controller to be
/// backend-agnostic.
pub trait Backend: Send {
    /// Emit a progress update.
    fn emit(&mut self, state: ProgressState, percent: Option<u8>, label: &str);

    /// Clear/remove the progress indicator.
    fn clear(&mut self);

    /// Returns the backend name for diagnostics.
    fn name(&self) -> &'static str;
}
