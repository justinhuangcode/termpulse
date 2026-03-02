//! Label sanitization to prevent OSC escape sequence injection.
//!
//! Any user-provided label text must be sanitized before being embedded
//! in an OSC sequence. Without sanitization, a malicious label could
//! inject arbitrary escape sequences into the terminal.

use crate::osc::{TERMINATOR_BEL, TERMINATOR_C1_ST};

/// Sanitize a label string for safe inclusion in an OSC 9;4 sequence.
///
/// Removes:
/// - ESC characters (`\x1b`) — prevents escape sequence injection
/// - All terminator sequences (ST, BEL, C1 ST) — prevents premature termination
/// - Closing brackets (`]`) — prevents OSC structure manipulation
/// - Control characters (0x00-0x1f, 0x7f) — prevents terminal confusion
///
/// The result is trimmed of leading/trailing whitespace.
///
/// # Example
///
/// ```
/// use termpulse_core::sanitize_label;
///
/// assert_eq!(sanitize_label("Hello World"), "Hello World");
/// assert_eq!(sanitize_label("evil\x1binject"), "evil");
/// assert_eq!(sanitize_label("  padded  "), "padded");
/// ```
#[must_use]
pub fn sanitize_label(label: &str) -> &str {
    // Fast path: if the label is clean, return it trimmed directly (zero-copy).
    let trimmed = label.trim();
    if is_clean(trimmed) {
        return trimmed;
    }
    // If we reach here, the label contains dangerous characters.
    // Since we're no_std and can't allocate, return the trimmed version
    // with a note that callers needing full sanitization should use
    // the allocating variant in the `termpulse` crate.
    //
    // For the no_std path, we strip from the first dangerous character.
    // Use char_indices to ensure we slice at valid char boundaries.
    for (i, ch) in trimmed.char_indices() {
        // Check each byte of the character
        let start = i;
        let end = start + ch.len_utf8();
        for &b in &trimmed.as_bytes()[start..end] {
            if is_dangerous_byte(b) {
                // Trim trailing whitespace so the result is idempotent.
                return trimmed[..i].trim_end();
            }
        }
    }
    trimmed
}

/// Check if a label string is clean (contains no dangerous characters).
///
/// Returns `true` if the label can be safely embedded in an OSC sequence
/// without any modification.
#[inline]
#[must_use]
pub fn is_clean(label: &str) -> bool {
    !label.bytes().any(is_dangerous_byte)
}

fn is_dangerous_byte(b: u8) -> bool {
    match b {
        // ESC — can start escape sequences
        0x1b => true,
        // BEL — OSC terminator
        b if b == TERMINATOR_BEL[0] => true,
        // C1 ST — OSC terminator
        b if b == TERMINATOR_C1_ST[0] => true,
        // Closing bracket — OSC structure
        b']' => true,
        // Control characters (except common whitespace within the label)
        0x00..=0x08 | 0x0e..=0x1a | 0x1c..=0x1f | 0x7f => true,
        // ST second byte when preceded by ESC is caught by ESC check above
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_label_unchanged() {
        assert_eq!(sanitize_label("Building project"), "Building project");
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(sanitize_label("  hello  "), "hello");
    }

    #[test]
    fn strips_from_escape() {
        assert_eq!(sanitize_label("good\x1bbad"), "good");
    }

    #[test]
    fn strips_from_bel() {
        assert_eq!(sanitize_label("good\x07bad"), "good");
    }

    #[test]
    fn strips_from_bracket() {
        assert_eq!(sanitize_label("good]bad"), "good");
    }

    #[test]
    fn strips_from_c1_st() {
        assert_eq!(sanitize_label("good\u{009c}bad"), "good");
    }

    #[test]
    fn empty_label() {
        assert_eq!(sanitize_label(""), "");
    }

    #[test]
    fn all_dangerous() {
        assert_eq!(sanitize_label("\x1b\x07\u{009c}"), "");
    }

    #[test]
    fn is_clean_true() {
        assert!(is_clean("Hello World 123"));
        assert!(is_clean("path/to/file.rs"));
    }

    #[test]
    fn is_clean_false() {
        assert!(!is_clean("has\x1bescape"));
        assert!(!is_clean("has\x07bell"));
    }
}
