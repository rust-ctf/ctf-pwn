use ascii::FromAsciiError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipeError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),
    #[error("ascii parse error {0}")]
    AsciiParseError(String),
    #[error("utf8 parse error {0}")]
    Utf8ParseError(#[from] std::string::FromUtf8Error),
    #[error("format error {0}")]
    FmtError(#[from] std::fmt::Error),
    #[error("regex error {0}")]
    RegexError(#[from] regex::Error),
    #[error("recv timeout")]
    Timeout,
    #[error("recv timeout")]
    ConditionFailed,
    #[error("unknown error")]
    Unknown,
}

impl<T> From<FromAsciiError<T>> for PipeError {
    fn from(value: FromAsciiError<T>) -> Self {
        PipeError::AsciiParseError(format!("{value}"))
    }
}
