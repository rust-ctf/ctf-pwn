use std::io;
use thiserror::Error;

/// Errors from I/O operations with timeouts.
#[derive(Error, Debug)]
pub enum IOTimeoutError {
    /// An I/O error occurred.
    #[error("IO Error {0}")]
    IOError(#[from] io::Error),
    /// The operation timed out.
    #[error("Timeout")]
    Timeout,
    /// The stream ended unexpectedly.
    #[error("Early eof")]
    UnexpectedEof,
}
