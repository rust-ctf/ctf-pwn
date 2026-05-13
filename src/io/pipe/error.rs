use std::io;
use thiserror::Error;

use crate::io::timeout::IOTimeoutError;

/// Errors that can occur during pipe operations.
#[derive(Error, Debug)]
pub enum PipeError {
    /// An I/O error occurred.
    #[error("IO Error {0}")]
    IOError(io::Error),
    /// The operation timed out.
    #[error("Timeout")]
    Timeout,
    /// The stream ended unexpectedly.
    #[error("Early eof")]
    UnexpectedEof,
    /// An unknown error occurred.
    #[error("Early eof")]
    Unknown,
}

impl From<io::Error> for PipeError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEof,
            io::ErrorKind::TimedOut => Self::Timeout,
            _ => Self::IOError(value),
        }
    }
}

impl From<IOTimeoutError> for PipeError {
    fn from(value: IOTimeoutError) -> Self {
        match value {
            IOTimeoutError::IOError(e) => e.into(),
            IOTimeoutError::Timeout => Self::Timeout,
            IOTimeoutError::UnexpectedEof => Self::UnexpectedEof,
        }
    }
}
