//! Async I/O primitives for ctf-pwn.
//!
//! This module provides the core building blocks for runtime-agnostic async
//! I/O: traits, buffering, timeouts, adapters, and transports.

/// Error types for I/O operations.
pub mod error;

/// Poll-based async read trait.
pub mod read;

/// Poll-based async write trait.
pub mod write;

/// Combined read/write trait for bidirectional I/O.
pub mod io_trait;

/// Runtime-agnostic timer abstraction.
pub mod timer;
