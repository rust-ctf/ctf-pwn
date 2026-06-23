//! Buffered async reader with peek/restore capabilities.

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::io::read::Read;

/// Trait for types that maintain an internal read buffer.
///
/// Enables peek-ahead and push-back operations required by structured
/// read methods like `read_until`.
pub trait Buffered: Read {
    /// View currently buffered data.
    fn buffer(&self) -> &[u8];

    /// Discard the first `n` bytes from the buffer.
    fn consume(&mut self, n: usize);

    /// Push data back to the front of the buffer (unread).
    fn restore(&mut self, data: &[u8]);

    /// Attempt to read more data from the inner reader into the buffer.
    ///
    /// Returns the number of new bytes added. A return of `0` indicates EOF.
    fn poll_fill(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<usize, Self::Error>>;
}

/// Async reader wrapper that maintains an internal `Vec<u8>` buffer.
///
/// Data is served from the buffer first. When the buffer is empty,
/// reads pass through to the inner reader.
#[derive(Debug)]
pub struct BufferedReader<R> {
    inner: R,
    buf: Vec<u8>,
}

impl<R> BufferedReader<R> {
    /// Create a new `BufferedReader` wrapping the given reader.
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: Vec::new(),
        }
    }

    /// View up to `n` bytes of buffered data without consuming.
    pub fn peek(&self, n: usize) -> &[u8] {
        let end = n.min(self.buf.len());
        &self.buf[..end]
    }

    /// Unwrap and return the inner reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R> Unpin for BufferedReader<R> {}

impl<R: Read + Unpin> Read for BufferedReader<R> {
    type Error = R::Error;

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        if !self.buf.is_empty() {
            let n = self.buf.len().min(buf.len());
            buf[..n].copy_from_slice(&self.buf[..n]);
            self.buf.drain(..n);
            return Poll::Ready(Ok(n));
        }
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<R: Read + Unpin> Buffered for BufferedReader<R> {
    fn buffer(&self) -> &[u8] {
        &self.buf
    }

    fn consume(&mut self, n: usize) {
        let n = n.min(self.buf.len());
        self.buf.drain(..n);
    }

    fn restore(&mut self, data: &[u8]) {
        self.buf.splice(0..0, data.iter().copied());
    }

    fn poll_fill(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<usize, Self::Error>> {
        let mut tmp = [0u8; 4096];
        match Pin::new(&mut self.inner).poll_read(cx, &mut tmp) {
            Poll::Ready(Ok(0)) => Poll::Ready(Ok(0)),
            Poll::Ready(Ok(n)) => {
                self.buf.extend_from_slice(&tmp[..n]);
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use crate::io::runtime_test::runtime_test;
    use crate::io::test_utils::{MockReader, ReadAction};

    use super::*;

    runtime_test!(read_drains_buffer_first, {
        let inner = MockReader::new(&[ReadAction::Data(b"inner".to_vec())]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"buffered");

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"buffered");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"inner");
    });

    runtime_test!(read_delegates_when_buffer_empty, {
        let inner = MockReader::new(&[ReadAction::Data(b"direct".to_vec())]);
        let mut reader = BufferedReader::new(inner);

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"direct");
    });

    runtime_test!(read_partial_buffer_drain, {
        let inner = MockReader::new(&[]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"abcdef");

        let mut buf = [0u8; 3];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"abc");
        assert_eq!(reader.buffer(), b"def");
    });

    runtime_test!(peek_returns_buffered_data, {
        let inner = MockReader::new(&[]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"hello world");

        assert_eq!(reader.peek(5), b"hello");
        assert_eq!(reader.peek(100), b"hello world");
        assert_eq!(reader.peek(0), b"");
    });

    runtime_test!(consume_removes_front_bytes, {
        let inner = MockReader::new(&[]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"abcdef");

        reader.consume(3);
        assert_eq!(reader.buffer(), b"def");

        reader.consume(100);
        assert_eq!(reader.buffer(), b"");
    });

    runtime_test!(restore_prepends_data, {
        let inner = MockReader::new(&[]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"world");
        reader.restore(b"hello ");

        assert_eq!(reader.buffer(), b"hello world");
    });

    runtime_test!(fill_reads_from_inner_into_buffer, {
        let inner = MockReader::new(&[ReadAction::Data(b"chunk".to_vec())]);
        let mut reader = BufferedReader::new(inner);

        assert!(reader.buffer().is_empty());

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(n, 5);
        assert_eq!(reader.buffer(), b"chunk");
    });

    runtime_test!(fill_appends_to_existing_buffer, {
        let inner = MockReader::new(&[ReadAction::Data(b"more".to_vec())]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"existing");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(n, 4);
        assert_eq!(reader.buffer(), b"existingmore");
    });

    runtime_test!(fill_returns_zero_on_eof, {
        let inner = MockReader::new(&[]);
        let mut reader = BufferedReader::new(inner);

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(n, 0);
    });

    runtime_test!(fill_with_sleep_between_chunks, {
        let inner = MockReader::new(&[
            ReadAction::Data(b"first".to_vec()),
            ReadAction::Sleep(Duration::from_millis(10)),
            ReadAction::Data(b"second".to_vec()),
        ]);
        let mut reader = BufferedReader::new(inner);

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(n, 5);
        assert_eq!(reader.buffer(), b"first");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(n, 6);
        assert_eq!(reader.buffer(), b"firstsecond");
    });

    runtime_test!(read_then_restore_then_read, {
        let inner = MockReader::new(&[
            ReadAction::Data(b"hello world".to_vec()),
        ]);
        let mut reader = BufferedReader::new(inner);

        let mut buf = [0u8; 5];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"hello");

        reader.restore(b"hello");
        assert_eq!(reader.buffer(), b"hello");

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"hello");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b" world");
    });

    runtime_test!(multiple_fill_calls_accumulate, {
        let inner = MockReader::new(&[
            ReadAction::Data(b"aaa".to_vec()),
            ReadAction::Data(b"bbb".to_vec()),
            ReadAction::Data(b"ccc".to_vec()),
        ]);
        let mut reader = BufferedReader::new(inner);

        core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();

        assert_eq!(reader.buffer(), b"aaabbbccc");
    });

    runtime_test!(consume_then_fill_then_read, {
        let inner = MockReader::new(&[ReadAction::Data(b"new".to_vec())]);
        let mut reader = BufferedReader::new(inner);
        reader.restore(b"old_data");

        reader.consume(4);
        assert_eq!(reader.buffer(), b"data");

        core::future::poll_fn(|cx| Pin::new(&mut reader).poll_fill(cx)).await.unwrap();
        assert_eq!(reader.buffer(), b"datanew");

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await.unwrap();
        assert_eq!(&buf[..n], b"datanew");
    });
}
