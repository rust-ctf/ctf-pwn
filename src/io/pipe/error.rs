use std::io;
use thiserror::Error;

use crate::io::timeout::IOTimeoutError;

#[derive(Error, Debug)]
pub enum PipeError {
    #[error("IO Error {0}")]
    IOError(io::Error),
    #[error("Timeout")]
    Timeout,
    #[error("Early eof")]
    UnexpectedEof,
}

impl From<io::Error> for PipeError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => PipeError::UnexpectedEof,
            io::ErrorKind::TimedOut => PipeError::Timeout,
            _ => PipeError::IOError(value),
        }
    }
}

impl From<IOTimeoutError> for PipeError {
    fn from(value: IOTimeoutError) -> Self {
        match value {
            IOTimeoutError::IOError(e) => e.into(),
            IOTimeoutError::Timeout => PipeError::Timeout,
            IOTimeoutError::UnexpectedEof => PipeError::UnexpectedEof,
        }
    }
}
