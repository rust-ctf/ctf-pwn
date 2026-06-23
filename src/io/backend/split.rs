//! Split I/O backend with separate reader and writer halves.

use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::io::io_trait::Io;
use crate::io::read::Read;
use crate::io::write::Write;

pin_project! {
    /// Two independent owned halves for bidirectional I/O.
    ///
    /// Used for TCP (split into read/write halves), process I/O
    /// (stdout/stdin), or split UART peripherals.
    #[derive(Debug)]
    pub struct Split<R, W> {
        #[pin]
        reader: R,
        #[pin]
        writer: W,
    }
}

impl<R, W> Split<R, W> {
    /// Create a new `Split` from separate reader and writer.
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    /// Returns a reference to the reader half.
    pub fn reader(&self) -> &R {
        &self.reader
    }

    /// Returns a reference to the writer half.
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Returns a mutable reference to the reader half.
    pub fn reader_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns a mutable reference to the writer half.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Decompose into the reader and writer halves.
    pub fn into_parts(self) -> (R, W) {
        (self.reader, self.writer)
    }
}

impl<R, W> Io for Split<R, W>
where
    R: Read,
    W: Write<Error = R::Error>,
{
    type Error = R::Error;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        self.project().reader.poll_read(cx, buf)
    }

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>> {
        self.project().writer.poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().writer.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().writer.poll_shutdown(cx)
    }
}

#[cfg(test)]
mod tests {
    use core::task::Waker;

    use crate::io::test_utils::{MockReader, MockWriter, ReadAction};

    use super::*;

    fn reader(actions: &[ReadAction]) -> MockReader {
        MockReader::new(actions)
    }

    fn writer() -> MockWriter {
        MockWriter::new()
    }

    fn data(d: &[u8]) -> ReadAction {
        ReadAction::Data(d.to_vec())
    }

    #[test]
    fn read_delegates_to_reader() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[data(b"hello")]), writer());
        let mut buf = [0u8; 64];

        let Poll::Ready(Ok(5)) = Pin::new(&mut split).poll_read(&mut cx, &mut buf) else {
            panic!("expected 5 bytes");
        };
        assert_eq!(&buf[..5], b"hello");
    }

    #[test]
    fn read_multiple_chunks() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(
            reader(&[data(b"hel"), data(b"lo")]),
            writer(),
        );
        let mut buf = [0u8; 64];

        let Poll::Ready(Ok(3)) = Pin::new(&mut split).poll_read(&mut cx, &mut buf) else {
            panic!("expected 3 bytes");
        };
        assert_eq!(&buf[..3], b"hel");

        let Poll::Ready(Ok(2)) = Pin::new(&mut split).poll_read(&mut cx, &mut buf) else {
            panic!("expected 2 bytes");
        };
        assert_eq!(&buf[..2], b"lo");
    }

    #[test]
    fn write_delegates_to_writer() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[]), writer());

        let result = Pin::new(&mut split).poll_write(&mut cx, b"world");
        assert!(matches!(result, Poll::Ready(Ok(5))));
        assert_eq!(split.writer().written, b"world");
    }

    #[test]
    fn flush_delegates_to_writer() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[]), writer());

        let result = Pin::new(&mut split).poll_flush(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));
    }

    #[test]
    fn shutdown_delegates_to_writer() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[]), writer());

        let result = Pin::new(&mut split).poll_shutdown(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));
    }

    #[test]
    fn into_parts_returns_both_halves() {
        let split = Split::new(reader(&[data(b"abc")]), writer());
        let (_reader, writer) = split.into_parts();
        assert!(writer.written.is_empty());
    }

    #[test]
    fn read_and_write_are_independent() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[data(b"input")]), writer());

        let mut buf = [0u8; 5];
        let read_result = Pin::new(&mut split).poll_read(&mut cx, &mut buf);
        assert!(matches!(read_result, Poll::Ready(Ok(5))));
        assert_eq!(&buf, b"input");

        let write_result = Pin::new(&mut split).poll_write(&mut cx, b"output");
        assert!(matches!(write_result, Poll::Ready(Ok(6))));
        assert_eq!(split.writer().written, b"output");
    }

    #[test]
    fn reader_eof_returns_zero() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut split = Split::new(reader(&[]), writer());
        let mut buf = [0u8; 64];

        let Poll::Ready(Ok(0)) = Pin::new(&mut split).poll_read(&mut cx, &mut buf) else {
            panic!("expected EOF");
        };
    }
}
