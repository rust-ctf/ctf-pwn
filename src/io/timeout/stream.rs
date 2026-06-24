use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use crate::io::error::Error;
use crate::io::read::Read;
use crate::io::timer::{Sleep, Timer, TimerProvider};
use crate::io::write::Write;

/// Wraps a type implementing both [`Read`] and [`Write`] with per-operation timeouts.
#[expect(missing_debug_implementations, reason = "Sleep future may not impl Debug")]
pub struct TimeoutStream<T> {
    inner: T,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    read_deadline: Option<Sleep>,
    write_deadline: Option<Sleep>,
}

impl<T> TimeoutStream<T> {
    /// Create a new `TimeoutStream` with the same timeout for reads and writes.
    pub fn new(inner: T, timeout: Option<Duration>) -> Self {
        Self {
            inner,
            read_timeout: timeout,
            write_timeout: timeout,
            read_deadline: None,
            write_deadline: None,
        }
    }

    /// Set the read timeout. Resets any active read deadline.
    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.read_timeout = timeout;
        self.read_deadline = None;
    }

    /// Set the write timeout. Resets any active write deadline.
    pub fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.write_timeout = timeout;
        self.write_deadline = None;
    }

    /// Returns the current read timeout.
    pub fn read_timeout(&self) -> Option<Duration> {
        self.read_timeout
    }

    /// Returns the current write timeout.
    pub fn write_timeout(&self) -> Option<Duration> {
        self.write_timeout
    }

    /// Returns a reference to the inner I/O object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the inner I/O object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Unwrap and return the inner I/O object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Unpin for TimeoutStream<T> {}

impl<T: Read + Unpin> Read for TimeoutStream<T> {
    type Error = Error<T::Error>;

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        match Pin::new(&mut self.inner).poll_read(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.read_deadline = None;
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => {
                self.read_deadline = None;
                Poll::Ready(Err(Error::Io(e)))
            }
            Poll::Pending => {
                let Some(timeout) = self.read_timeout else {
                    return Poll::Pending;
                };
                let deadline =
                    self.read_deadline.get_or_insert_with(|| Timer::sleep(timeout));
                match Pin::new(deadline).poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(()) => {
                        self.read_deadline = None;
                        Poll::Ready(Err(Error::Timeout))
                    }
                }
            }
        }
    }
}

impl<T: Write + Unpin> Write for TimeoutStream<T> {
    type Error = Error<T::Error>;

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>> {
        match Pin::new(&mut self.inner).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.write_deadline = None;
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => {
                self.write_deadline = None;
                Poll::Ready(Err(Error::Io(e)))
            }
            Poll::Pending => {
                let Some(timeout) = self.write_timeout else {
                    return Poll::Pending;
                };
                let deadline =
                    self.write_deadline.get_or_insert_with(|| Timer::sleep(timeout));
                match Pin::new(deadline).poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(()) => {
                        self.write_deadline = None;
                        Poll::Ready(Err(Error::Timeout))
                    }
                }
            }
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner)
            .poll_flush(cx)
            .map_err(Error::Io)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner)
            .poll_shutdown(cx)
            .map_err(Error::Io)
    }
}
