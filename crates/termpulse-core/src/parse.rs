//! OSC 9;4 sequence parser.
//!
//! Finds and parses OSC 9;4 progress sequences embedded in arbitrary text.
//! Useful for extracting progress information from terminal output streams.

use crate::osc::{
    OSC_PREFIX, ProgressState, TERMINATOR_BEL, TERMINATOR_C1_ST, TERMINATOR_ST, Terminator,
};

/// A parsed OSC 9;4 sequence found within text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ParsedSequence {
    /// Byte offset where the sequence starts in the source text.
    pub start: usize,
    /// Byte offset where the sequence ends (exclusive) in the source text.
    pub end: usize,
    /// The parsed progress state.
    pub state: ProgressState,
    /// The parsed percentage, if present.
    pub percent: Option<u8>,
    /// Byte range of the label within the source text, if present.
    ///
    /// Use `source[label_start..label_end]` to extract.
    pub label_start: usize,
    /// End of label byte range (exclusive).
    pub label_end: usize,
    /// Which terminator was used.
    pub terminator: Terminator,
}

impl ParsedSequence {
    /// An empty/zeroed `ParsedSequence` for initializing output buffers.
    ///
    /// Use this to create arrays for [`find_sequences`]:
    ///
    /// ```
    /// use termpulse_core::{ParsedSequence, find_sequences};
    ///
    /// let mut buf = [ParsedSequence::EMPTY; 8];
    /// let count = find_sequences(b"some text", &mut buf);
    /// ```
    pub const EMPTY: Self = Self {
        start: 0,
        end: 0,
        state: ProgressState::Clear,
        percent: None,
        label_start: 0,
        label_end: 0,
        terminator: Terminator::St,
    };

    /// Returns `true` if this sequence has a non-empty label.
    #[inline]
    #[must_use]
    pub const fn has_label(&self) -> bool {
        self.label_end > self.label_start
    }
}

/// Find all OSC 9;4 sequences in the given text.
///
/// Returns an iterator-like result. Since we're `no_std`, we collect into
/// a caller-provided buffer.
///
/// # Returns
///
/// The number of sequences found and written to `out`.
///
/// # Example
///
/// ```
/// use termpulse_core::{find_sequences, ParsedSequence};
///
/// let text = "hello \x1b]9;4;1;50;Building\x1b\\ world";
/// let mut buf = [ParsedSequence::EMPTY; 8];
/// let count = find_sequences(text.as_bytes(), &mut buf);
/// assert_eq!(count, 1);
/// assert_eq!(buf[0].percent, Some(50));
/// ```
#[must_use]
pub fn find_sequences(input: &[u8], out: &mut [ParsedSequence]) -> usize {
    let mut count = 0;
    let mut pos = 0;

    while pos < input.len() && count < out.len() {
        // Find next OSC prefix
        let Some(prefix_pos) = find_prefix(input, pos) else {
            break;
        };

        let seq_start = prefix_pos;
        let after_prefix = prefix_pos + OSC_PREFIX.len();

        // Parse state digit
        if after_prefix >= input.len() {
            break;
        }
        let state_byte = input[after_prefix];
        let Some(state) = ProgressState::from_u8(state_byte.wrapping_sub(b'0')) else {
            pos = after_prefix + 1;
            continue;
        };

        // Expect semicolon after state
        let mut cursor = after_prefix + 1;
        if cursor >= input.len() || input[cursor] != b';' {
            pos = cursor;
            continue;
        }
        cursor += 1;

        // Parse percent (may be empty)
        let percent_start = cursor;
        while cursor < input.len() && input[cursor].is_ascii_digit() {
            cursor += 1;
        }
        let percent = if cursor > percent_start {
            let mut val: u16 = 0;
            for &b in &input[percent_start..cursor] {
                val = val * 10 + u16::from(b - b'0');
            }
            if val <= 100 {
                Some(val as u8)
            } else {
                pos = cursor;
                continue;
            }
        } else {
            None
        };

        // Expect semicolon before label
        if cursor >= input.len() || input[cursor] != b';' {
            pos = cursor;
            continue;
        }
        cursor += 1;

        // Find the nearest terminator
        let label_start = cursor;
        let Some((term_pos, terminator, term_len)) = find_nearest_terminator(input, cursor) else {
            // Unterminated sequence
            break;
        };

        let label_end = term_pos;
        let seq_end = term_pos + term_len;

        out[count] = ParsedSequence {
            start: seq_start,
            end: seq_end,
            state,
            percent,
            label_start,
            label_end,
            terminator,
        };
        count += 1;
        pos = seq_end;
    }

    count
}

fn find_prefix(input: &[u8], start: usize) -> Option<usize> {
    let prefix = OSC_PREFIX;
    if input.len() < prefix.len() {
        return None;
    }
    for i in start..=input.len() - prefix.len() {
        if &input[i..i + prefix.len()] == prefix {
            return Some(i);
        }
    }
    None
}

fn find_nearest_terminator(input: &[u8], start: usize) -> Option<(usize, Terminator, usize)> {
    let mut best: Option<(usize, Terminator, usize)> = None;

    // Check ST: \x1b\\
    if let Some(p) = find_bytes(input, start, TERMINATOR_ST) {
        best = Some((p, Terminator::St, TERMINATOR_ST.len()));
    }

    // Check BEL: \x07
    if let Some(p) = find_bytes(input, start, TERMINATOR_BEL) {
        if best.is_none() || p < best.unwrap().0 {
            best = Some((p, Terminator::Bel, TERMINATOR_BEL.len()));
        }
    }

    // Check C1 ST: \x9c
    if let Some(p) = find_bytes(input, start, TERMINATOR_C1_ST) {
        if best.is_none() || p < best.unwrap().0 {
            best = Some((p, Terminator::C1St, TERMINATOR_C1_ST.len()));
        }
    }

    best
}

fn find_bytes(haystack: &[u8], start: usize, needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    for i in start..=haystack.len() - needle.len() {
        if &haystack[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_parsed() -> ParsedSequence {
        ParsedSequence {
            start: 0,
            end: 0,
            state: ProgressState::Clear,
            percent: None,
            label_start: 0,
            label_end: 0,
            terminator: Terminator::St,
        }
    }

    #[test]
    fn parse_normal_with_label() {
        let input = b"\x1b]9;4;1;50;Building\x1b\\";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 1);
        assert_eq!(buf[0].state, ProgressState::Normal);
        assert_eq!(buf[0].percent, Some(50));
        assert_eq!(
            core::str::from_utf8(&input[buf[0].label_start..buf[0].label_end]).unwrap(),
            "Building"
        );
    }

    #[test]
    fn parse_indeterminate() {
        let input = b"\x1b]9;4;3;;Waiting\x1b\\";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 1);
        assert_eq!(buf[0].state, ProgressState::Indeterminate);
        assert_eq!(buf[0].percent, None);
    }

    #[test]
    fn parse_clear() {
        let input = b"\x1b]9;4;0;0;\x1b\\";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 1);
        assert_eq!(buf[0].state, ProgressState::Clear);
        assert_eq!(buf[0].percent, Some(0));
    }

    #[test]
    fn parse_bel_terminator() {
        let input = b"\x1b]9;4;1;75;test\x07";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 1);
        assert_eq!(buf[0].terminator, Terminator::Bel);
        assert_eq!(buf[0].percent, Some(75));
    }

    #[test]
    fn parse_multiple_sequences() {
        let input = b"prefix\x1b]9;4;1;25;A\x1b\\middle\x1b]9;4;1;75;B\x1b\\suffix";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 2);
        assert_eq!(buf[0].percent, Some(25));
        assert_eq!(buf[1].percent, Some(75));
    }

    #[test]
    fn parse_no_sequences() {
        let input = b"just regular text";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 0);
    }

    #[test]
    fn parse_embedded_in_text() {
        let input = b"hello \x1b]9;4;1;42;test\x1b\\ world";
        let mut buf = [empty_parsed(); 4];
        let count = find_sequences(input, &mut buf);
        assert_eq!(count, 1);
        assert_eq!(buf[0].start, 6);
        assert_eq!(buf[0].percent, Some(42));
    }
}
