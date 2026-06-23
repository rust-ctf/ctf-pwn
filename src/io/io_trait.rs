//! Combined read/write trait for bidirectional I/O.
//!
//! The [`Io`] trait is what [`Session`](crate::session::Session) is generic
//! over. It is implemented by [`Split<R, W>`](crate::io::backend::Split) for
//! separate reader/writer halves and [`Duplex<RW>`](crate::io::backend::Duplex)
//! for a single bidirectional object.

use core::pin::Pin;
use core::task::{Context, Poll};

/// Bidirectional async I/O with a unified error type.
pub trait Io {
    /// The error type for both read and write operations.
    type Error;

    /// Attempt to read bytes into `buf`.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;

    /// Attempt to write bytes from `buf`.
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
