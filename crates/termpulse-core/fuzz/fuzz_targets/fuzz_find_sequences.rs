#![no_main]

use libfuzzer_sys::fuzz_target;
use termpulse_core::{ParsedSequence, ProgressState, Terminator, find_sequences};

fuzz_target!(|data: &[u8]| {
    let mut out = [ParsedSequence {
        start: 0,
        end: 0,
        state: ProgressState::Clear,
        percent: None,
        label_start: 0,
        label_end: 0,
        terminator: Terminator::St,
    }; 16];

    let count = find_sequences(data, &mut out);

    // Invariant: count never exceeds buffer length.
    assert!(count <= out.len());

    // Invariant: parsed ranges are within input bounds.
    for parsed in &out[..count] {
        assert!(parsed.start <= parsed.end);
        assert!(parsed.end <= data.len());
        assert!(parsed.label_start <= parsed.label_end);
        assert!(parsed.label_end <= data.len());
    }
});
