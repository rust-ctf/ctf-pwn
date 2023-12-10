use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep_until, Instant, Sleep};

#[derive(Debug)]
pub(crate) struct TimeoutState {
    timeout: Option<Duration>,
    cur: Pin<Box<Sleep>>,
    active: bool,
}

impl TimeoutState {
    #[inline]
    pub(crate) fn new() -> TimeoutState {
        TimeoutState {
            timeout: None,
            cur: Box::pin(sleep_until(Instant::now())),
            active: false,
        }
    }

    #[inline]
    pub(crate) fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    #[inline]
    pub(crate) fn set_timeout(&mut self, timeout: Option<Duration>) {
        // since this takes &mut self, we can't yet be active
        self.timeout = timeout;
    }

    #[inline]
    pub(crate) fn set_timeout_pinned(mut self: Pin<&mut Self>, timeout: Option<Duration>) {
        self.as_mut().timeout = timeout;
        self.reset();
    }

    #[inline]
    pub(crate) fn reset(mut self: Pin<&mut Self>) {
        if self.active {
            self.active = false;
            self.cur.as_mut().reset(Instant::now());
        }
    }

    #[inline]
    pub(crate) fn poll_check(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> io::Result<()> {
        let timeout = match self.timeout {
            Some(timeout) => timeout,
            None => return Ok(()),
        };

        if !self.active {
            self.cur.as_mut().reset(Instant::now() + timeout);
            self.active = true;
        }

        match self.cur.as_mut().poll(cx) {
            Poll::Ready(()) => Err(io::Error::from(io::ErrorKind::TimedOut)),
            Poll::Pending => Ok(()),
        }
    }
}
