//! Error types for ctf-pwn operations.

use core::fmt;

/// Unified error type for all I/O operations.
///
/// Generic over `E`, the underlying transport error type (e.g. `std::io::Error`
/// for tokio/async-std, or a HAL error for embassy).
#[derive(Debug)]
pub enum Error<E> {
    /// An error from the underlying transport.
    Io(E),
    /// The operation exceeded its deadline.
    Timeout,
    /// The stream ended before the operation could complete.
    UnexpectedEof,
    /// A regex pattern failed to compile.
    #[cfg(feature = "regex")]
    InvalidPattern(alloc::string::String),
}

impl<E: fmt::Debug> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e:?}"),
            Self::Timeout => write!(f, "operation timed out"),
            Self::UnexpectedEof => write!(f, "unexpected end of stream"),
            #[cfg(feature = "regex")]
            Self::InvalidPattern(p) => write!(f, "invalid regex pattern: {p}"),
        }
    }
}

impl<E> From<E> for Error<E> {
    fn from(err: E) -> Self {
        Self::Io(err)
    }
}
