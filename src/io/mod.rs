//! I/O module providing pipes, caching readers, and timeout utilities.

/// Cached async reader support.
pub mod cache;
/// Pipe abstractions for process and TCP communication.
pub mod pipe;
#[cfg(test)]
/// Test utilities for simulating async readers.
pub mod test;
/// Timeout wrappers for async read operations.
pub mod timeout;
