use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use crate::io::error::Error;
use crate::io::timer::{Sleep, Timer, TimerProvider};
use crate::io::write::Write;

/// Wraps a [`Write`] type with a per-operation timeout.
#[expect(missing_debug_implementations, reason = "Sleep future may not impl Debug")]
pub struct TimeoutWriter<W> {
    inner: W,
    timeout: Option<Duration>,
    deadline: Option<Sleep>,
}

impl<W> TimeoutWriter<W> {
    /// Create a new `TimeoutWriter`.
    pub fn new(inner: W, timeout: Option<Duration>) -> Self {
        Self {
            inner,
            timeout,
            deadline: None,
        }
    }

    /// Set the timeout. Resets any active deadline.
    pub fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
        self.deadline = None;
    }

    /// Returns the current timeout.
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Returns a reference to the inner writer.
    pub fn inner(&self) -> &W {
        &self.inner
    }

    /// Returns a mutable reference to the inner writer.
    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Unwrap and return the inner writer.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W> Unpin for TimeoutWriter<W> {}

impl<W: Write + Unpin> Write for TimeoutWriter<W> {
    type Error = Error<W::Error>;

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>> {
        match Pin::new(&mut self.inner).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.deadline = None;
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => {
                self.deadline = None;
                Poll::Ready(Err(Error::Io(e)))
            }
            Poll::Pending => {
                let Some(timeout) = self.timeout else {
                    return Poll::Pending;
                };
                let deadline = self.deadline.get_or_insert_with(|| Timer::sleep(timeout));
                match Pin::new(deadline).poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(()) => {
                        self.deadline = None;
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
