use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, BufReader, ReadBuf};
use tokio::time::{timeout_at, Instant};
use std::future::Future;

pub struct TimedBufReader<R: AsyncRead> {
    reader: BufReader<R>,
    timeout: Duration,
}

impl<R: AsyncRead> TimedBufReader<R> {
    pub fn new(inner: R, timeout: Duration) -> Self {
        TimedBufReader {
            reader: BufReader::new(inner),
            timeout,
        }
    }

    pub fn from_buf(reader: BufReader<R>, timeout: Duration) -> Self {
        TimedBufReader {
            reader,
            timeout,
        }
    }

    pub fn get_timeout(&self) -> Duration
    {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: Duration)
    {
        self.timeout = timeout;
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for TimedBufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let timeout = self.timeout;
        let mut read_fut = Box::pin(self.reader.read_buf(buf));
        
        let mut timeout_future = Box::pin(timeout_at(Instant::now() + timeout, &mut read_fut));
        
        match timeout_future.as_mut().poll(cx) {
            Poll::Ready(Ok(Ok(_result))) => Poll::Ready(Ok(())),
            Poll::Ready(Ok(Err(e))) => Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, e))),
            Poll::Ready(Err(_)) => {
                Poll::Ready(Err(io::Error::new(io::ErrorKind::TimedOut, "read operation timed out")))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
