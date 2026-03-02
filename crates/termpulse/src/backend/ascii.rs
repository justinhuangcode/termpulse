//! ASCII fallback backend — visual progress bar on stderr.
//!
//! Used when the terminal is a TTY but doesn't support OSC 9;4.
//! Renders a classic `[=====>     ] 50% Building` progress bar.

use super::Backend;
use crate::ProgressState;
use std::io::Write;

const BAR_WIDTH: usize = 30;

/// ASCII progress bar backend.
pub struct AsciiBackend<W: Write> {
    writer: W,
    last_line_len: usize,
}

impl<W: Write> AsciiBackend<W> {
    /// Create a new ASCII backend writing to the given writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            last_line_len: 0,
        }
    }
}

impl AsciiBackend<std::io::Stderr> {
    /// Create an ASCII backend writing to stderr.
    pub fn stderr() -> Self {
        Self::new(std::io::stderr())
    }
}

impl<W: Write + Send> Backend for AsciiBackend<W> {
    fn emit(&mut self, state: ProgressState, percent: Option<u8>, label: &str) {
        let line = match state {
            ProgressState::Clear => String::new(),
            ProgressState::Indeterminate => {
                if label.is_empty() {
                    "[ <=>                          ] ...".to_string()
                } else {
                    format!("[ <=>                          ] {label}")
                }
            }
            _ => {
                let p = percent.unwrap_or(0) as usize;
                let filled = (p * BAR_WIDTH) / 100;
                let empty = BAR_WIDTH.saturating_sub(filled).saturating_sub(1);

                let state_indicator = match state {
                    ProgressState::Error => "!",
                    ProgressState::Paused => "~",
                    _ => ">",
                };

                let bar: String = "=".repeat(filled);
                let space: String = " ".repeat(empty);

                if label.is_empty() {
                    format!("[{bar}{state_indicator}{space}] {p:>3}%")
                } else {
                    format!("[{bar}{state_indicator}{space}] {p:>3}% {label}")
                }
            }
        };

        // Overwrite previous line with carriage return
        let padding = if line.len() < self.last_line_len {
            " ".repeat(self.last_line_len - line.len())
        } else {
            String::new()
        };

        let _ = write!(self.writer, "\r{line}{padding}");
        let _ = self.writer.flush();
        self.last_line_len = line.len();
    }

    fn clear(&mut self) {
        if self.last_line_len > 0 {
            let blank = " ".repeat(self.last_line_len);
            let _ = write!(self.writer, "\r{blank}\r");
            let _ = self.writer.flush();
            self.last_line_len = 0;
        }
    }

    fn name(&self) -> &'static str {
        "ascii"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_progress_bar() {
        let mut buf = Vec::new();
        let mut backend = AsciiBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(50), "Building");
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("==="));
        assert!(output.contains("50%"));
        assert!(output.contains("Building"));
    }

    #[test]
    fn renders_zero_percent() {
        let mut buf = Vec::new();
        let mut backend = AsciiBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(0), "Starting");
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("0%"));
    }

    #[test]
    fn renders_hundred_percent() {
        let mut buf = Vec::new();
        let mut backend = AsciiBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(100), "Done");
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("100%"));
    }

    #[test]
    fn renders_indeterminate() {
        let mut buf = Vec::new();
        let mut backend = AsciiBackend::new(&mut buf);
        backend.emit(ProgressState::Indeterminate, None, "Waiting");
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("<=>"));
        assert!(output.contains("Waiting"));
    }

    #[test]
    fn clears_line() {
        let mut buf = Vec::new();
        let mut backend = AsciiBackend::new(&mut buf);
        backend.emit(ProgressState::Normal, Some(50), "test");
        backend.clear();
        let output = String::from_utf8(buf).unwrap();
        // Should end with \r (carriage return to clear)
        assert!(output.ends_with('\r'));
    }
}
