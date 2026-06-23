//! CTF pwn utilities for Rust.
//!
//! Runtime-agnostic async I/O primitives for interacting with CTF challenge
//! services, hardware targets, and local binaries.
//!
//! # Feature flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `std` | Standard library support (default) |
//! | `alloc` | Heap allocation for buffers (implied by `std`) |
//! | `runtime-tokio` | Tokio runtime support (default) |
//! | `runtime-async-std` | async-std runtime support |
//! | `runtime-smol` | Smol runtime support |
//! | `runtime-embassy` | Embassy runtime support for embedded targets |
//! | `regex` | Regex-based read operations (default) |
//! | `tui` | Interactive terminal UI |

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

/// Async I/O primitives: traits, buffering, timeouts, adapters, transports.
pub mod io;
