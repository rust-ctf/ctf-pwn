//! Async I/O primitives for ctf-pwn.
//!
//! This module provides the core building blocks for runtime-agnostic async
//! I/O: traits, buffering, timeouts, adapters, and transports.

const _RUNTIME_COUNT: u8 = cfg!(feature = "runtime-tokio") as u8
    + cfg!(feature = "runtime-async-std") as u8
    + cfg!(feature = "runtime-smol") as u8
    + cfg!(feature = "runtime-embassy") as u8;

const _: () = assert!(
    _RUNTIME_COUNT <= 1,
    "Only one `runtime-*` feature may be enabled at a time."
);

const _: () = assert!(
    _RUNTIME_COUNT >= 1,
    "At least one `runtime-*` feature must be enabled."
);

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

/// I/O backends for bidirectional communication.
pub mod backend;

#[cfg(test)]
mod runtime_test;

#[cfg(test)]
pub(crate) mod test_utils;
