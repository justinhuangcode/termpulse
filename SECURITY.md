# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in termpulse, please report it
responsibly.

**Do not open a public issue.**

Instead, email **justinhuangcode@gmail.com** with:

- A description of the vulnerability
- Steps to reproduce
- Affected versions
- Any suggested fix (optional)

You should receive an acknowledgement within 48 hours. We will work with you
to understand and address the issue before any public disclosure.

## Scope

termpulse processes terminal escape sequences and user-provided label text.
Security-relevant areas include:

- **Label sanitization** — preventing OSC injection via `sanitize_label`
- **Sequence parsing** — preventing buffer overflows or malformed input crashes
- **Signal handling** — ensuring clean shutdown in `wrap` command

## Security Design

- `termpulse-core` uses `#![deny(unsafe_code)]` — no unsafe Rust
- All label text is sanitized before embedding in escape sequences
- The parser operates on bounded buffers with no heap allocation
- Property-based tests (proptest) fuzz sanitization invariants continuously
