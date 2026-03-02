# termpulse-core

[![Crates.io](https://img.shields.io/crates/v/termpulse-core?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/termpulse-core)
[![docs.rs](https://img.shields.io/docsrs/termpulse-core?style=flat-square&logo=docs.rs&logoColor=white)](https://docs.rs/termpulse-core)
[![License](https://img.shields.io/crates/l/termpulse-core?style=flat-square)](../../LICENSE)

Core OSC 9;4 terminal progress protocol -- build, parse, and sanitize escape sequences.

**`no_std` compatible, zero dependencies, zero allocations.**

## Usage

```rust
use termpulse_core::{OscSequence, ProgressState, Terminator};

// Build a sequence
let seq = OscSequence::normal_with_label(50, "Building");
let mut buf = [0u8; 256];
let n = seq.write_to(&mut buf).unwrap();
// buf[..n] = b"\x1b]9;4;1;50;Building\x1b\\"

// Parse sequences from raw bytes
use termpulse_core::{ParsedSequence, find_sequences};
let mut out = [ParsedSequence::EMPTY; 8];
let count = find_sequences(&buf[..n], &mut out);
assert_eq!(count, 1);

// Sanitize labels (prevent escape injection)
use termpulse_core::sanitize_label;
let clean = sanitize_label("evil\x1b]inject");
assert_eq!(clean, "evil");
```

## Feature flags

| Feature | Default | Description |
| --- | --- | --- |
| `std` | off | Implements `std::error::Error` for `WriteError` |

## Part of termpulse

This is the low-level core crate. For the full library with terminal detection, backends, throttling, and ETA estimation, see [termpulse](https://crates.io/crates/termpulse). For the CLI, see [termpulse-cli](https://crates.io/crates/termpulse-cli).

## License

[MIT](../../LICENSE)
