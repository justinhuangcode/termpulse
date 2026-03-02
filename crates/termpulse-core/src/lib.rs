//! # termpulse-core
//!
//! Core OSC 9;4 terminal progress protocol implementation.
//!
//! This crate provides the lowest-level building blocks for working with
//! OSC 9;4 terminal progress sequences:
//!
//! - **Build** escape sequences from structured data
//! - **Parse** raw bytes back into structured progress state
//! - **Sanitize** labels to prevent escape sequence injection
//! - **Strip** progress sequences from text for clean logging
//!
//! ## Design Principles
//!
//! - `no_std` compatible — works in embedded, WASM, and FFI contexts
//! - Zero dependencies — nothing to audit, nothing to break
//! - Zero allocations in core paths — all operations work on slices
//!
//! ## OSC 9;4 Protocol
//!
//! ```text
//! ESC ] 9;4;<state>;<percent>;<label> <terminator>
//!
//! State:      0=clear  1=normal  2=error  3=indeterminate  4=paused
//! Percent:    0-100 (omitted for indeterminate)
//! Label:      Optional descriptive text
//! Terminator: ST(\x1b\\) or BEL(\x07) or C1_ST(\x9c)
//! ```
//!
//! ## Example
//!
//! ```
//! use termpulse_core::OscSequence;
//!
//! let seq = OscSequence::normal_with_label(42, "Building");
//!
//! let mut buf = [0u8; 128];
//! let n = seq.write_to(&mut buf).unwrap();
//! assert_eq!(&buf[..n], b"\x1b]9;4;1;42;Building\x1b\\");
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/termpulse-core/0.1.0")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "std")]
extern crate std;

mod osc;
mod parse;
mod sanitize;
mod strip;

pub use osc::{OSC_PREFIX, TERMINATOR_BEL, TERMINATOR_C1_ST, TERMINATOR_ST};
pub use osc::{OscSequence, ProgressState, Terminator, WriteError};
pub use parse::{ParsedSequence, find_sequences};
pub use sanitize::{is_clean, sanitize_label};
pub use strip::strip_sequences;
