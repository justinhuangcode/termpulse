//! Strip OSC 9;4 sequences from text.
//!
//! Useful for cleaning terminal output before writing to log files or
//! displaying in contexts that don't support escape sequences.

use crate::osc::{ProgressState, Terminator};
use crate::parse::{ParsedSequence, find_sequences};

/// Strip all OSC 9;4 progress sequences from the input bytes.
///
/// Returns the number of bytes written to `out`.
/// The caller must provide an output buffer at least as large as `input`.
///
/// # Example
///
/// ```
/// use termpulse_core::strip_sequences;
///
/// let input = b"hello \x1b]9;4;1;50;test\x1b\\ world";
/// let mut out = [0u8; 256];
/// let n = strip_sequences(input, &mut out);
/// assert_eq!(&out[..n], b"hello  world");
/// ```
#[must_use]
pub fn strip_sequences(input: &[u8], out: &mut [u8]) -> usize {
    // Find all sequences first
    let empty = ParsedSequence {
        start: 0,
        end: 0,
        state: ProgressState::Clear,
        percent: None,
        label_start: 0,
        label_end: 0,
        terminator: Terminator::St,
    };
    let mut sequences = [empty; 32]; // Support up to 32 sequences per strip
    let count = find_sequences(input, &mut sequences);

    if count == 0 {
        // No sequences found, copy input as-is
        let len = input.len().min(out.len());
        out[..len].copy_from_slice(&input[..len]);
        return len;
    }

    let mut out_pos = 0;
    let mut in_pos = 0;

    for seq in &sequences[..count] {
        // Copy bytes before this sequence
        if seq.start > in_pos {
            let chunk = &input[in_pos..seq.start];
            let end = out_pos + chunk.len();
            if end <= out.len() {
                out[out_pos..end].copy_from_slice(chunk);
                out_pos = end;
            }
        }
        in_pos = seq.end;
    }

    // Copy remaining bytes after last sequence
    if in_pos < input.len() {
        let chunk = &input[in_pos..];
        let end = out_pos + chunk.len();
        if end <= out.len() {
            out[out_pos..end].copy_from_slice(chunk);
            out_pos = end;
        }
    }

    out_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_single_sequence() {
        let input = b"hello \x1b]9;4;1;50;test\x1b\\ world";
        let mut out = [0u8; 256];
        let n = strip_sequences(input, &mut out);
        assert_eq!(&out[..n], b"hello  world");
    }

    #[test]
    fn strip_no_sequences() {
        let input = b"plain text";
        let mut out = [0u8; 256];
        let n = strip_sequences(input, &mut out);
        assert_eq!(&out[..n], b"plain text");
    }

    #[test]
    fn strip_only_sequence() {
        let input = b"\x1b]9;4;1;100;done\x1b\\";
        let mut out = [0u8; 256];
        let n = strip_sequences(input, &mut out);
        assert_eq!(n, 0);
    }

    #[test]
    fn strip_multiple_sequences() {
        let input = b"a\x1b]9;4;1;25;x\x1b\\b\x1b]9;4;1;75;y\x1b\\c";
        let mut out = [0u8; 256];
        let n = strip_sequences(input, &mut out);
        assert_eq!(&out[..n], b"abc");
    }
}
