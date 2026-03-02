//! OSC 9;4 backend — native terminal progress indicators.

use super::Backend;
use crate::ProgressState;
use std::io::Write;
use termpulse_core::{OscSequence, Terminator, sanitize_label};

/// OSC 9;4 backend that writes escape sequences to a writer (typically stderr).
pub struct OscBackend<W: Write> {
    writer: W,
}

impl<W: Write> OscBackend<W> {
    /// Create a new OSC backend writing to the given writer.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl OscBackend<std::io::Stderr> {
    /// Create an OSC backend writing to stderr (the standard choice).
    pub fn stderr() -> Self {
        Self::new(std::io::stderr())
    }
}

impl<W: Write + Send> Backend for OscBackend<W> {
    fn emit(&mut self, state: ProgressState, percent: Option<u8>, label: &str) {
        let sanitized = sanitize_label(label);
        let label_opt = if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        };

        let seq = OscSequence {
            state,
            percent,
            label: label_opt,
            terminator: Terminator::St,
        };

        let mut buf = [0u8; 256];
        if let Ok(n) = seq.write_to(&mut buf) {
            // Ignore write errors — progress is best-effort
            let _ = self.writer.write_all(&buf[..n]);
            let _ = self.writer.flush();
        }
    }

    fn clear(&mut self) {
        let seq = OscSequence::clear();
        let mut buf = [0u8; 64];
        if let Ok(n) = seq.write_to(&mut buf) {
            let _ = self.writer.write_all(&buf[..n]);
            let _ = self.writer.flush();
        }
    }

    fn name(&self) -> &'static str {
        "osc"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_osc_sequence() {
        let mut buf = Vec::new();
        let mut backend = OscBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(50), "Building");
        assert_eq!(buf, b"\x1b]9;4;1;50;Building\x1b\\");
    }

    #[test]
    fn clears_progress() {
        let mut buf = Vec::new();
        let mut backend = OscBackend::new(&mut buf);
        backend.clear();
        assert_eq!(buf, b"\x1b]9;4;0;0;\x1b\\");
    }

    #[test]
    fn sanitizes_label() {
        let mut buf = Vec::new();
        let mut backend = OscBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(10), "evil\x1binject");
        // Label should be truncated at the escape character
        assert_eq!(buf, b"\x1b]9;4;1;10;evil\x1b\\");
    }
}
