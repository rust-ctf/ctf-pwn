//! Poll-based async write trait.
//!
//! Generic error type for `no_std` compatibility.

use core::pin::Pin;
use core::task::{Context, Poll};

/// Async writer with a generic error type.
pub trait Write {
    /// The error type for write operations.
    type Error;

    /// Attempt to write bytes from `buf`.
    ///
    /// On success, returns `Poll::Ready(Ok(n))` where `n` is the number of
    /// bytes written. A return value of `0` typically indicates that the
    /// writer can no longer accept data.
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>>;

    /// Attempt to flush any buffered output.
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    /// Attempt to shut down the write side of the transport.
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;
}
