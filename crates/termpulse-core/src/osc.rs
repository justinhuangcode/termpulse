//! OSC 9;4 sequence construction.
//!
//! Provides types and constants for building valid OSC 9;4 terminal progress
//! escape sequences without any heap allocation.

/// OSC 9;4 prefix: `ESC ] 9;4;`
pub const OSC_PREFIX: &[u8] = b"\x1b]9;4;";

/// String Terminator: `ESC \`
pub const TERMINATOR_ST: &[u8] = b"\x1b\\";

/// Bell terminator: `BEL`
pub const TERMINATOR_BEL: &[u8] = b"\x07";

/// C1 String Terminator
pub const TERMINATOR_C1_ST: &[u8] = b"\x9c";

/// Progress indicator state.
///
/// Maps directly to the OSC 9;4 state parameter:
/// - `0` = clear/remove the progress indicator
/// - `1` = normal progress (default)
/// - `2` = error state (typically rendered red)
/// - `3` = indeterminate (spinner, no percentage)
/// - `4` = paused/warning (semantics vary by terminal)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
#[repr(u8)]
pub enum ProgressState {
    /// Remove/clear the progress indicator.
    Clear = 0,
    /// Normal active progress.
    #[default]
    Normal = 1,
    /// Error state (typically red).
    Error = 2,
    /// Indeterminate progress (spinner-like, no percentage).
    Indeterminate = 3,
    /// Paused or warning state.
    ///
    /// Note: semantics for state 4 are ambiguous across terminals.
    /// Some treat it as "paused", others as "warning".
    Paused = 4,
}

impl ProgressState {
    /// Convert a raw `u8` to a `ProgressState`, if valid.
    #[inline]
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Clear),
            1 => Some(Self::Normal),
            2 => Some(Self::Error),
            3 => Some(Self::Indeterminate),
            4 => Some(Self::Paused),
            _ => None,
        }
    }
}

impl From<ProgressState> for u8 {
    #[inline]
    fn from(state: ProgressState) -> Self {
        state as Self
    }
}

impl TryFrom<u8> for ProgressState {
    type Error = u8;

    /// Converts a raw `u8` to a `ProgressState`.
    ///
    /// Returns `Err(value)` if the value does not map to a known state.
    #[inline]
    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// Sequence terminator variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Terminator {
    /// ST (String Terminator): `ESC \` — most widely supported.
    #[default]
    St,
    /// BEL: `\x07` — alternative terminator.
    Bel,
    /// C1 ST: `\x9c` — single-byte variant.
    C1St,
}

impl Terminator {
    /// Returns the raw bytes for this terminator.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        match self {
            Self::St => TERMINATOR_ST,
            Self::Bel => TERMINATOR_BEL,
            Self::C1St => TERMINATOR_C1_ST,
        }
    }
}

/// Errors that can occur when writing an OSC sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WriteError {
    /// The output buffer is too small.
    BufferTooSmall {
        /// Required buffer size in bytes.
        required: usize,
        /// Actual buffer size provided.
        available: usize,
    },
    /// Percent value is out of range (must be 0..=100).
    PercentOutOfRange(u8),
}

impl core::fmt::Display for WriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall {
                required,
                available,
            } => write!(
                f,
                "buffer too small: need {required} bytes, have {available}"
            ),
            Self::PercentOutOfRange(v) => {
                write!(f, "percent out of range: {v} (must be 0..=100)")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WriteError {}

#[cfg(feature = "std")]
impl From<WriteError> for std::io::Error {
    fn from(err: WriteError) -> Self {
        Self::other(err)
    }
}

/// A structured OSC 9;4 progress sequence.
///
/// This type represents a complete OSC 9;4 escape sequence that can be
/// written to a byte buffer without allocation.
///
/// # Example
///
/// ```
/// use termpulse_core::{OscSequence, ProgressState, Terminator};
///
/// let seq = OscSequence {
///     state: ProgressState::Normal,
///     percent: Some(75),
///     label: Some("Downloading"),
///     terminator: Terminator::St,
/// };
///
/// let mut buf = [0u8; 128];
/// let n = seq.write_to(&mut buf).unwrap();
/// let output = core::str::from_utf8(&buf[..n]).unwrap();
/// assert_eq!(output, "\x1b]9;4;1;75;Downloading\x1b\\");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OscSequence<'a> {
    /// The progress state.
    pub state: ProgressState,
    /// Progress percentage (0-100). `None` for indeterminate or clear.
    pub percent: Option<u8>,
    /// Optional label text. Will be sanitized before writing.
    pub label: Option<&'a str>,
    /// Which terminator to use.
    pub terminator: Terminator,
}

impl<'a> OscSequence<'a> {
    /// Create a simple normal progress sequence.
    #[must_use]
    pub const fn normal(percent: u8) -> Self {
        Self {
            state: ProgressState::Normal,
            percent: Some(percent),
            label: None,
            terminator: Terminator::St,
        }
    }

    /// Create a normal progress sequence with a label.
    #[must_use]
    pub const fn normal_with_label(percent: u8, label: &'a str) -> Self {
        Self {
            state: ProgressState::Normal,
            percent: Some(percent),
            label: Some(label),
            terminator: Terminator::St,
        }
    }

    /// Create an indeterminate progress sequence.
    #[must_use]
    pub const fn indeterminate(label: &'a str) -> Self {
        Self {
            state: ProgressState::Indeterminate,
            percent: None,
            label: Some(label),
            terminator: Terminator::St,
        }
    }

    /// Create a clear/remove progress sequence.
    #[must_use]
    pub const fn clear() -> Self {
        Self {
            state: ProgressState::Clear,
            percent: Some(0),
            label: None,
            terminator: Terminator::St,
        }
    }

    /// Create an error state sequence.
    #[must_use]
    pub const fn error(label: &'a str) -> Self {
        Self {
            state: ProgressState::Error,
            percent: None,
            label: Some(label),
            terminator: Terminator::St,
        }
    }

    /// Calculate the exact byte length this sequence will occupy.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        let mut len = OSC_PREFIX.len(); // \x1b]9;4;

        // state digit
        len += 1; // single digit 0-4

        // ;percent
        len += 1; // semicolon
        if let Some(p) = self.percent {
            if p >= 100 {
                len += 3;
            } else if p >= 10 {
                len += 2;
            } else {
                len += 1;
            }
        }

        // ;label
        len += 1; // semicolon
        if let Some(label) = self.label {
            len += label.len(); // upper bound (sanitized may be shorter)
        }

        // terminator
        len += self.terminator.as_bytes().len();

        len
    }

    /// Write this sequence into a byte buffer.
    ///
    /// Returns the number of bytes written on success.
    ///
    /// # Errors
    ///
    /// Returns [`WriteError::BufferTooSmall`] if `buf` is too small.
    /// Returns [`WriteError::PercentOutOfRange`] if percent > 100.
    pub fn write_to(&self, buf: &mut [u8]) -> Result<usize, WriteError> {
        if let Some(p) = self.percent {
            if p > 100 {
                return Err(WriteError::PercentOutOfRange(p));
            }
        }

        let required = self.byte_len();
        if buf.len() < required {
            return Err(WriteError::BufferTooSmall {
                required,
                available: buf.len(),
            });
        }

        let mut pos = 0;

        // Write prefix: \x1b]9;4;
        let prefix = OSC_PREFIX;
        buf[pos..pos + prefix.len()].copy_from_slice(prefix);
        pos += prefix.len();

        // Write state digit
        buf[pos] = b'0' + (self.state as u8);
        pos += 1;

        // Write ;percent
        buf[pos] = b';';
        pos += 1;
        if let Some(p) = self.percent {
            if p >= 100 {
                buf[pos] = b'1';
                buf[pos + 1] = b'0';
                buf[pos + 2] = b'0';
                pos += 3;
            } else if p >= 10 {
                buf[pos] = b'0' + (p / 10);
                buf[pos + 1] = b'0' + (p % 10);
                pos += 2;
            } else {
                buf[pos] = b'0' + p;
                pos += 1;
            }
        }

        // Write ;label
        buf[pos] = b';';
        pos += 1;
        if let Some(label) = self.label {
            let label_bytes = label.as_bytes();
            buf[pos..pos + label_bytes.len()].copy_from_slice(label_bytes);
            pos += label_bytes.len();
        }

        // Write terminator
        let term = self.terminator.as_bytes();
        buf[pos..pos + term.len()].copy_from_slice(term);
        pos += term.len();

        Ok(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_progress() {
        let seq = OscSequence::normal_with_label(50, "Building");
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;1;50;Building\x1b\\");
    }

    #[test]
    fn clear_progress() {
        let seq = OscSequence::clear();
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;0;0;\x1b\\");
    }

    #[test]
    fn indeterminate_progress() {
        let seq = OscSequence::indeterminate("Waiting");
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;3;;Waiting\x1b\\");
    }

    #[test]
    fn error_progress() {
        let seq = OscSequence::error("Failed");
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;2;;Failed\x1b\\");
    }

    #[test]
    fn bel_terminator() {
        let seq = OscSequence {
            state: ProgressState::Normal,
            percent: Some(99),
            label: None,
            terminator: Terminator::Bel,
        };
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;1;99;\x07");
    }

    #[test]
    fn percent_boundaries() {
        // 0%
        let seq = OscSequence::normal(0);
        let mut buf = [0u8; 128];
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;1;0;\x1b\\");

        // 100%
        let seq = OscSequence::normal(100);
        let n = seq.write_to(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"\x1b]9;4;1;100;\x1b\\");
    }

    #[test]
    fn percent_out_of_range() {
        let seq = OscSequence::normal(101);
        let mut buf = [0u8; 128];
        assert_eq!(
            seq.write_to(&mut buf),
            Err(WriteError::PercentOutOfRange(101))
        );
    }

    #[test]
    fn buffer_too_small() {
        let seq = OscSequence::normal(50);
        let mut buf = [0u8; 2];
        assert!(matches!(
            seq.write_to(&mut buf),
            Err(WriteError::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn all_states_roundtrip() {
        for state_val in 0..=4u8 {
            let state = ProgressState::from_u8(state_val).unwrap();
            assert_eq!(state as u8, state_val);
        }
        assert!(ProgressState::from_u8(5).is_none());
    }
}
