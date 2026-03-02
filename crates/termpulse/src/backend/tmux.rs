//! tmux DCS passthrough backend.
//!
//! When running inside tmux, OSC sequences must be wrapped in a DCS
//! passthrough envelope to reach the outer terminal:
//!
//! ```text
//! \ePtmux;\e<osc-sequence>\e\\
//! ```
//!
//! tmux 3.3+ supports `allow-passthrough` which lets applications send
//! escape sequences directly to the outer terminal.

use super::Backend;
use crate::ProgressState;
use std::io::Write;
use termpulse_core::{OscSequence, Terminator, sanitize_label};

/// DCS prefix for tmux passthrough.
const DCS_TMUX_START: &[u8] = b"\x1bPtmux;";
/// DCS string terminator.
const DCS_TMUX_END: &[u8] = b"\x1b\\";

/// OSC backend wrapped in tmux DCS passthrough.
///
/// Wraps each OSC 9;4 sequence in `\ePtmux;\e...\e\\` so that tmux
/// forwards it to the outer terminal.
pub struct TmuxBackend<W: Write> {
    writer: W,
}

impl<W: Write> TmuxBackend<W> {
    /// Create a new tmux passthrough backend writing to the given writer.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl TmuxBackend<std::io::Stderr> {
    /// Create a tmux passthrough backend writing to stderr.
    pub fn stderr() -> Self {
        Self::new(std::io::stderr())
    }
}

impl<W: Write + Send> Backend for TmuxBackend<W> {
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

        let mut osc_buf = [0u8; 256];
        if let Ok(n) = seq.write_to(&mut osc_buf) {
            // Wrap in DCS passthrough: ESC inside the sequence must be doubled
            // for tmux: each \x1b becomes \x1b\x1b
            let _ = self.writer.write_all(DCS_TMUX_START);
            for &b in &osc_buf[..n] {
                if b == 0x1b {
                    let _ = self.writer.write_all(b"\x1b");
                }
                let _ = self.writer.write_all(&[b]);
            }
            let _ = self.writer.write_all(DCS_TMUX_END);
            let _ = self.writer.flush();
        }
    }

    fn clear(&mut self) {
        let seq = OscSequence::clear();
        let mut osc_buf = [0u8; 64];
        if let Ok(n) = seq.write_to(&mut osc_buf) {
            let _ = self.writer.write_all(DCS_TMUX_START);
            for &b in &osc_buf[..n] {
                if b == 0x1b {
                    let _ = self.writer.write_all(b"\x1b");
                }
                let _ = self.writer.write_all(&[b]);
            }
            let _ = self.writer.write_all(DCS_TMUX_END);
            let _ = self.writer.flush();
        }
    }

    fn name(&self) -> &'static str {
        "osc-tmux"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_in_dcs_passthrough() {
        let mut buf = Vec::new();
        let mut backend = TmuxBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(50), "test");

        // Expected: \ePtmux;\e\e]9;4;1;50;test\e\e\\\e\\
        // The inner \e is doubled for each ESC in the OSC sequence
        let expected = b"\x1bPtmux;\x1b\x1b]9;4;1;50;test\x1b\x1b\\\x1b\\";
        assert_eq!(buf, expected);
    }

    #[test]
    fn clear_wraps_in_dcs() {
        let mut buf = Vec::new();
        let mut backend = TmuxBackend::new(&mut buf);
        backend.clear();

        let expected = b"\x1bPtmux;\x1b\x1b]9;4;0;0;\x1b\x1b\\\x1b\\";
        assert_eq!(buf, expected);
    }
}
