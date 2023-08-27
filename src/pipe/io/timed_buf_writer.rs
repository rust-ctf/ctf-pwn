use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use std::future::Future;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};
use tokio::time::Instant;

pub struct TimedBufWriter<W: AsyncWrite> {
    writer: BufWriter<W>,
    timeout: Duration,
    start_time: Option<Instant>,
    poll_rate: Duration,
}

impl<W: AsyncWrite> TimedBufWriter<W> {
    pub fn new(inner: W, timeout: Duration) -> Self {
        TimedBufWriter {
            writer: BufWriter::new(inner),
            timeout,
            start_time: None,
            poll_rate: Duration::from_millis(50),
        }
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn get_poll_rate(&self) -> Duration {
        self.timeout
    }

    pub fn set_poll_rate(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    fn clear_watch(&mut self) {
        self.start_time = None;
    }

    fn start_watch(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    fn is_timeout(&self) -> bool {
        match self.start_time {
            None => false,
            Some(start_time) => (Instant::now() - start_time) > self.timeout,
        }
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for TimedBufWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.is_timeout() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "read operation timed out",
            )));
        }

        self.start_watch();

        let mut write_fut = Box::pin(self.writer.write(buf));

        match write_fut.as_mut().poll(cx) {
            Poll::Ready(Ok(result)) => {
                self.clear_watch();
                Poll::Ready(Ok(result))
            },
            Poll::Ready(Err(e)) => {
                self.clear_watch();
                Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, e)))
            },
            Poll::Pending => {
                let waker = cx.waker().clone();
                let delay_duration = self.poll_rate; // adjust as needed
                tokio::spawn(async move {
                    tokio::time::sleep(delay_duration).await;
                    waker.wake();
                });
                Poll::Pending
            }
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}
