use ascii::FromAsciiError;
use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipeError {
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("ascii parse error")]
    AsciiParseError(String),
    #[error("utf8 parse error")]
    Utf8ParseError(#[from] std::string::FromUtf8Error),
    #[error("unknown error")]
    Unknown,
}

impl<T> From<FromAsciiError<T>> for PipeError {
    fn from(value: FromAsciiError<T>) -> Self {
        PipeError::AsciiParseError(format!("{value}"))
    }
}

pub type Result<T> = result::Result<T, PipeError>;
