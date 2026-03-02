#![no_main]

use libfuzzer_sys::fuzz_target;
use termpulse_core::sanitize_label;

fuzz_target!(|data: &str| {
    let result = sanitize_label(data);

    // Invariant: no dangerous bytes in output.
    for b in result.bytes() {
        assert!(b != 0x1b, "ESC in sanitized output");
        assert!(b != 0x07, "BEL in sanitized output");
        assert!(b != 0x9c, "C1 ST in sanitized output");
        assert!(b != b']', "']' in sanitized output");
        assert!(b >= 0x20 || b == b'\t', "control char in sanitized output");
    }

    // Invariant: idempotent.
    let twice = sanitize_label(result);
    assert_eq!(result, twice, "sanitize_label is not idempotent");

    // Invariant: result is a substring of the trimmed input.
    let trimmed = data.trim();
    if !result.is_empty() {
        assert!(
            trimmed.contains(result),
            "sanitized result not found in trimmed input"
        );
    }
});
