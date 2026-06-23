//! Poll-based async read trait.
//!
//! This trait is the foundation for all read operations in ctf-pwn.
//! It uses a generic error type rather than `std::io::Error`, making it
//! compatible with `no_std` environments including embassy.

use core::pin::Pin;
use core::task::{Context, Poll};

/// Async reader with a generic error type.
///
/// Unlike `tokio::io::AsyncRead` or `futures::io::AsyncRead`, this trait
/// returns the number of bytes read directly and uses `&mut [u8]` instead
/// of `ReadBuf`.
pub trait Read {
    /// The error type for read operations.
    type Error;

    /// Attempt to read bytes into `buf`.
    ///
    /// On success, returns `Poll::Ready(Ok(n))` where `n` is the number of
    /// bytes read. A return value of `0` indicates EOF.
    ///
    /// If no data is currently available, returns `Poll::Pending` and
    /// arranges for the current task to be woken when data arrives.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;
}
