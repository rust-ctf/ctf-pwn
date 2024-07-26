use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IOTimeoutError {
    #[error("IO Error {0}")]
    IOError(#[from] io::Error),
    #[error("Timeout")]
    Timeout,
}
