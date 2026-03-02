//! Property-based tests for termpulse-core.
//!
//! These tests use proptest to verify invariants hold for arbitrary inputs.
//! They run with std available (integration test crate), so they can use
//! Vec/String even though the library is `no_std`.

use proptest::prelude::*;
use termpulse_core::{
    OscSequence, ParsedSequence, ProgressState, Terminator, find_sequences, sanitize_label,
    strip_sequences,
};

fn empty_parsed() -> ParsedSequence {
    ParsedSequence::EMPTY
}

// -- sanitize_label properties --

proptest! {
    /// sanitize_label output must never contain dangerous bytes.
    #[test]
    fn sanitize_never_contains_dangerous_bytes(input in "\\PC{0,200}") {
        let result = sanitize_label(&input);
        for b in result.bytes() {
            prop_assert!(b != 0x1b, "ESC found in sanitized label");
            prop_assert!(b != 0x07, "BEL found in sanitized label");
            prop_assert!(b != 0x9c, "C1 ST found in sanitized label");
            prop_assert!(b != b']', "']' found in sanitized label");
            prop_assert!(b >= 0x20 || b == b'\t', "control char {:#x} found in sanitized label", b);
        }
    }

    /// sanitize_label output is always a valid UTF-8 substring of the original
    /// (after trimming).
    #[test]
    fn sanitize_returns_valid_utf8_prefix(input in "\\PC{0,200}") {
        let result = sanitize_label(&input);
        // Must be valid UTF-8
        prop_assert!(std::str::from_utf8(result.as_bytes()).is_ok());
        // Must be a prefix of the trimmed input (zero-copy)
        let trimmed = input.trim();
        if !result.is_empty() {
            prop_assert!(trimmed.starts_with(result),
                "sanitized '{}' is not a prefix of trimmed '{}'", result, trimmed);
        }
    }

    /// sanitize_label is idempotent — sanitizing twice gives the same result.
    #[test]
    fn sanitize_is_idempotent(input in "\\PC{0,200}") {
        let once = sanitize_label(&input);
        let twice = sanitize_label(once);
        prop_assert_eq!(once, twice);
    }
}

// -- OscSequence write/parse round-trip properties --

proptest! {
    /// Any OscSequence that fits in the buffer can be written and parsed back.
    #[test]
    fn write_parse_roundtrip(
        state_val in 0u8..5,
        percent in proptest::option::of(0u8..=100),
        label in "[a-zA-Z0-9 _-]{0,50}",
    ) {
        let state = match state_val {
            0 => ProgressState::Clear,
            1 => ProgressState::Normal,
            2 => ProgressState::Error,
            3 => ProgressState::Indeterminate,
            _ => ProgressState::Paused,
        };

        let sanitized = sanitize_label(&label);
        let label_opt = if sanitized.is_empty() { None } else { Some(sanitized) };

        let seq = OscSequence {
            state,
            percent,
            label: label_opt,
            terminator: Terminator::St,
        };

        let mut buf = [0u8; 512];
        let n = seq.write_to(&mut buf).unwrap();

        // Parse it back
        let mut parsed = [empty_parsed(); 4];
        let count = find_sequences(&buf[..n], &mut parsed);

        prop_assert_eq!(count, 1, "expected exactly 1 parsed sequence, got {}", count);
        prop_assert_eq!(parsed[0].state, state);

        if let Some(p) = percent {
            prop_assert_eq!(parsed[0].percent, Some(p));
        }
    }
}

// -- strip_sequences properties --

proptest! {
    /// strip_sequences output never contains OSC 9;4 sequences.
    #[test]
    fn strip_removes_all_sequences(
        prefix in "[a-z]{0,20}",
        percent in 0u8..=100,
        suffix in "[a-z]{0,20}",
    ) {
        // Build input with an embedded sequence
        let seq = OscSequence::normal(percent);
        let mut seq_buf = [0u8; 64];
        let seq_len = seq.write_to(&mut seq_buf).unwrap();

        let mut input = Vec::new();
        input.extend_from_slice(prefix.as_bytes());
        input.extend_from_slice(&seq_buf[..seq_len]);
        input.extend_from_slice(suffix.as_bytes());

        let mut out = vec![0u8; input.len()];
        let n = strip_sequences(&input, &mut out);

        let stripped = &out[..n];

        // Verify no sequences remain
        let mut check = [empty_parsed(); 4];
        let remaining = find_sequences(stripped, &mut check);
        prop_assert_eq!(remaining, 0, "stripped output still contains sequences");

        // Verify prefix and suffix are preserved
        prop_assert!(stripped.starts_with(prefix.as_bytes()));
        prop_assert!(stripped.ends_with(suffix.as_bytes()));
    }
}
