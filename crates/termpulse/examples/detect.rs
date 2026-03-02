//! Detect terminal capabilities.
//!
//! Run with: `cargo run -p termpulse --example detect`

#![allow(clippy::print_stdout)]

use termpulse::detect::{self, DetectOptions, EnvReader, detect_multiplexer};

fn main() {
    let cap = detect::detect(&DetectOptions::default());
    let mux = detect_multiplexer(&EnvReader::REAL);

    println!("Detected capability: {cap:?}");
    println!("Multiplexer: {mux:?}");
}
