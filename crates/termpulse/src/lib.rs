//! # termpulse
//!
//! Native terminal progress indicators via OSC 9;4 — with smart terminal
//! detection, graceful fallback, throttling, and stall detection.
//!
//! ## Quick Start
//!
//! ```no_run
//! use termpulse::Controller;
//!
//! let mut ctrl = Controller::auto();
//! ctrl.set(50, "Building");
//! // ... do work ...
//! ctrl.done("Complete");
//! ```
//!
//! ## Architecture
//!
//! ```text
//! termpulse (this crate)
//!   ├── Controller     — stateful progress management
//!   ├── detect         — terminal capability detection
//!   ├── backend/       — output strategy (osc, ascii, silent)
//!   ├── throttle       — rate limiting & deduplication
//!   └── estimate       — ETA & throughput calculation
//!
//! termpulse-core (dependency)
//!   └── OSC 9;4 sequence build/parse/sanitize
//! ```
//!
//! ## Design Principles
//!
//! 1. **Zero-config**: `Controller::auto()` detects the best backend
//! 2. **Safe by default**: Labels are sanitized, percentages clamped
//! 3. **Graceful degradation**: OSC → ASCII → Silent (three-tier fallback)
//! 4. **No panic**: All operations are infallible from the caller's perspective
//! 5. **Dependency injection**: All I/O is injectable for testing

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/termpulse/0.1.0")]

pub mod backend;
mod controller;
pub mod detect;
pub mod estimate;
pub mod throttle;

pub use controller::Controller;
pub use detect::{Multiplexer, TerminalCapability};

// Re-export core types for convenience
pub use termpulse_core::{
    OscSequence, ProgressState, Terminator, find_sequences, is_clean, sanitize_label,
    strip_sequences,
};
