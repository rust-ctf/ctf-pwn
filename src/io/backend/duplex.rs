//! Duplex I/O backend for a single bidirectional object.

use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::io::io_trait::Io;
use crate::io::read::Read;
use crate::io::write::Write;

pin_project! {
    /// A single object that implements both [`Read`] and [`Write`].
    ///
    /// Used for unsplit UART peripherals, serial ports, or any transport
    /// where read and write go through the same underlying resource.
    #[derive(Debug)]
    pub struct Duplex<RW> {
        #[pin]
        inner: RW,
    }
}

impl<RW> Duplex<RW> {
    /// Create a new `Duplex` wrapping a bidirectional object.
    pub fn new(inner: RW) -> Self {
        Self { inner }
    }

    /// Returns a reference to the inner object.
    pub fn inner(&self) -> &RW {
        &self.inner
    }

    /// Returns a mutable reference to the inner object.
    pub fn inner_mut(&mut self) -> &mut RW {
        &mut self.inner
    }

    /// Unwrap and return the inner object.
    pub fn into_inner(self) -> RW {
        self.inner
    }
}

impl<RW> Io for Duplex<RW>
where
    RW: Read + Write<Error = <RW as Read>::Error>,
{
    type Error = <RW as Read>::Error;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        self.project().inner.poll_read(cx, buf)
    }

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_shutdown(cx)
    }
}

#[cfg(test)]
mod tests {
    use core::task::Waker;

    use crate::io::test_utils::{MockStream, ReadAction};

    use super::*;

    fn bidi(actions: &[ReadAction]) -> MockStream {
        MockStream::new(actions)
    }

    fn data(d: &[u8]) -> ReadAction {
        ReadAction::Data(d.to_vec())
    }

    #[test]
    fn read_delegates_to_inner() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[data(b"hello")]));
        let mut buf = [0u8; 5];

        let result = Pin::new(&mut duplex).poll_read(&mut cx, &mut buf);
        assert!(matches!(result, Poll::Ready(Ok(5))));
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn write_delegates_to_inner() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[]));

        let result = Pin::new(&mut duplex).poll_write(&mut cx, b"world");
        assert!(matches!(result, Poll::Ready(Ok(5))));
        assert_eq!(duplex.inner().writer.written, b"world");
    }

    #[test]
    fn read_and_write_through_same_object() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[data(b"input")]));

        let mut buf = [0u8; 5];
        let read_result = Pin::new(&mut duplex).poll_read(&mut cx, &mut buf);
        assert!(matches!(read_result, Poll::Ready(Ok(5))));
        assert_eq!(&buf, b"input");

        let write_result = Pin::new(&mut duplex).poll_write(&mut cx, b"output");
        assert!(matches!(write_result, Poll::Ready(Ok(6))));
        assert_eq!(duplex.inner().writer.written, b"output");
    }

    #[test]
    fn read_multiple_chunks() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[data(b"first"), data(b"second")]));
        let mut buf = [0u8; 64];

        let Poll::Ready(Ok(5)) = Pin::new(&mut duplex).poll_read(&mut cx, &mut buf) else {
            panic!("expected first chunk");
        };
        assert_eq!(&buf[..5], b"first");

        let Poll::Ready(Ok(6)) = Pin::new(&mut duplex).poll_read(&mut cx, &mut buf) else {
            panic!("expected second chunk");
        };
        assert_eq!(&buf[..6], b"second");
    }

    #[test]
    fn flush_delegates_to_inner() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[]));

        let result = Pin::new(&mut duplex).poll_flush(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));
    }

    #[test]
    fn shutdown_delegates_to_inner() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[]));

        let result = Pin::new(&mut duplex).poll_shutdown(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));
    }

    #[test]
    fn into_inner_returns_object() {
        let duplex = Duplex::new(bidi(&[data(b"abc")]));
        let inner = duplex.into_inner();
        assert!(inner.writer.written.is_empty());
    }

    #[test]
    fn eof_returns_zero() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut duplex = Duplex::new(bidi(&[]));
        let mut buf = [0u8; 8];

        let result = Pin::new(&mut duplex).poll_read(&mut cx, &mut buf);
        assert!(matches!(result, Poll::Ready(Ok(0))));
    }
}
