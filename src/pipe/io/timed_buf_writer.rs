use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use std::future::Future;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};
use tokio::time::{timeout_at, Instant};

pub struct TimedBufWriter<W: AsyncWrite> {
    writer: BufWriter<W>,
    timeout: Duration,
}

impl<W: AsyncWrite> TimedBufWriter<W> {
    pub fn new(inner: W, timeout: Duration) -> Self {
        TimedBufWriter {
            writer: BufWriter::new(inner),
            timeout,
        }
    }

    pub fn from_buf(writer: BufWriter<W>, timeout: Duration) -> Self {
        TimedBufWriter { writer, timeout }
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for TimedBufWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let timeout = self.timeout;
        let mut write_fut = Box::pin(self.writer.write(buf));

        let mut timeout_future = Box::pin(timeout_at(Instant::now() + timeout, &mut write_fut));

        match timeout_future.as_mut().poll(cx) {
            Poll::Ready(Ok(Ok(result))) => Poll::Ready(Ok(result)),
            Poll::Ready(Ok(Err(e))) => Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, e))),
            Poll::Ready(Err(_)) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "write operation timed out",
            ))),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}
