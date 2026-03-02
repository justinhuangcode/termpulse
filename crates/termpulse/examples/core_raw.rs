//! Direct usage of termpulse-core for low-level OSC 9;4 sequence construction.
//!
//! This example shows how to build and emit raw OSC sequences without the
//! high-level Controller. Useful for embedded or custom integrations.
//!
//! Run with: `cargo run -p termpulse --example core_raw`

#![allow(clippy::print_stdout, clippy::print_stderr)]

use termpulse::{OscSequence, ProgressState, Terminator};

fn main() {
    // Build a normal progress sequence at 42%
    let seq = OscSequence {
        state: ProgressState::Normal,
        percent: Some(42),
        label: Some("Downloading"),
        terminator: Terminator::St,
    };

    let mut buf = [0u8; 256];
    let n = seq.write_to(&mut buf).unwrap();

    // Write directly to stderr
    let _ = std::io::Write::write_all(&mut std::io::stderr(), &buf[..n]);
    let _ = std::io::Write::flush(&mut std::io::stderr());

    eprintln!();
    eprintln!("Wrote {} bytes: {:?}", n, std::str::from_utf8(&buf[..n]));

    // Clear
    let clear = OscSequence::clear();
    let n = clear.write_to(&mut buf).unwrap();
    let _ = std::io::Write::write_all(&mut std::io::stderr(), &buf[..n]);
}
