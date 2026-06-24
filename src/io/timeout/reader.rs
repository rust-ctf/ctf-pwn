use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use crate::io::error::Error;
use crate::io::read::Read;
use crate::io::timer::{Sleep, Timer, TimerProvider};

/// Wraps a [`Read`] type with a per-operation timeout.
#[expect(missing_debug_implementations, reason = "Sleep future may not impl Debug")]
pub struct TimeoutReader<R> {
    inner: R,
    timeout: Option<Duration>,
    deadline: Option<Sleep>,
}

impl<R> TimeoutReader<R> {
    /// Create a new `TimeoutReader`.
    pub fn new(inner: R, timeout: Option<Duration>) -> Self {
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

    /// Returns a reference to the inner reader.
    pub fn inner(&self) -> &R {
        &self.inner
    }

    /// Returns a mutable reference to the inner reader.
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Unwrap and return the inner reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R> Unpin for TimeoutReader<R> {}

impl<R: Read + Unpin> Read for TimeoutReader<R> {
    type Error = Error<R::Error>;

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        match Pin::new(&mut self.inner).poll_read(cx, buf) {
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
}
