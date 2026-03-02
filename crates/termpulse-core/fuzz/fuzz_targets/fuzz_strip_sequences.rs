#![no_main]

use libfuzzer_sys::fuzz_target;
use termpulse_core::{ParsedSequence, ProgressState, Terminator, find_sequences, strip_sequences};

fuzz_target!(|data: &[u8]| {
    let mut out = vec![0u8; data.len()];
    let n = strip_sequences(data, &mut out);

    // Invariant: output is never longer than input.
    assert!(n <= data.len());

    // Invariant: no sequences remain in stripped output.
    let stripped = &out[..n];
    let mut check = [ParsedSequence {
        start: 0,
        end: 0,
        state: ProgressState::Clear,
        percent: None,
        label_start: 0,
        label_end: 0,
        terminator: Terminator::St,
    }; 16];
    let remaining = find_sequences(stripped, &mut check);
    assert_eq!(remaining, 0, "stripped output still contains sequences");
});
