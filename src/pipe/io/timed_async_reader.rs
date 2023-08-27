use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use std::future::Future;
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};
use tokio::time::Instant;

use crate::RollbackReader;

pub struct TimedAsyncReader<R: AsyncRead> {
    reader: RollbackReader<R>,
    timeout: Duration,
    start_time: Option<Instant>,
    poll_rate: Duration,
}

impl<R: AsyncRead + Unpin> TimedAsyncReader<R> {
    pub fn new(inner: R, timeout: Duration) -> Self {
        TimedAsyncReader {
            reader: RollbackReader::new(inner),
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

impl<R: AsyncRead + Unpin> AsyncRead for TimedAsyncReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.is_timeout() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "read operation timed out",
            )));
        }

        self.start_watch();
        let mut read_fut = Box::pin(self.reader.read_buf(buf));
        match read_fut.as_mut().poll(cx) {
            Poll::Ready(Ok(_result)) => {
                self.clear_watch();
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                self.clear_watch();
                Poll::Ready(Err(e))
            }
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
}
