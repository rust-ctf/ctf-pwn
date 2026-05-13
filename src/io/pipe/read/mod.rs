//! Extension methods for reading from pipes.

mod recv;
mod recv_all;
mod recv_regex;
mod recv_until;
mod recv_until_regex;
mod result;
pub(crate) use recv::*;
pub(crate) use recv_all::*;
pub(crate) use recv_regex::*;
pub(crate) use recv_until::*;
pub(crate) use recv_until_regex::*;
pub(crate) use result::*;

use std::time::Duration;

use crate::io::timeout::PwnTimeout;

use super::PipeRead;

impl<R: PipeRead> PipeReadExt for R {}

/// Resolves the timeout delay, falling back to a far-future duration.
fn timeout_delay(timeout: Option<Duration>) -> Duration {
    match timeout {
        Some(delay) => delay,
        None => PwnTimeout::far_away(),
    }
}

/// Extension trait providing high-level receive operations on pipes.
pub trait PipeReadExt: PipeRead {
    /// Receive up to 4 KiB of data.
    fn recv(&mut self) -> Recv<'_, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv::recv(self, 1024 * 4, delay)
    }

    /// Receive up to `size` bytes of data.
    fn recvn(&mut self, size: usize) -> Recv<'_, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv::recv(self, size, delay)
    }

    /// Receive all data until EOF or timeout.
    fn recvall(&mut self) -> RecvAll<'_, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_all::recv_all(self, delay)
    }

    /// Receive data until the given delimiter is found.
    fn recvuntil<D: AsRef<[u8]>>(&mut self, delimiter: D) -> RecvUntil<'_, Self, D>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_until::recv_until(self, delimiter, delay)
    }

    /// Receive data until a regex pattern matches.
    ///
    /// # Errors
    ///
    /// Returns `regex::Error` if the pattern is invalid.
    fn recvregex<'a>(&'a mut self, pattern: &str) -> Result<RecvRegex<'a, Self>, regex::Error>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_regex::recv_regex(self, pattern, delay)
    }

    /// Receive data until a regex pattern matches, returning up to the match.
    ///
    /// # Errors
    ///
    /// Returns `regex::Error` if the pattern is invalid.
    fn recvuntilregex<'a>(
        &'a mut self,
        pattern: &str,
    ) -> Result<RecvUntilRegex<'a, Self>, regex::Error>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_until_regex::recv_until_regex(self, pattern, delay)
    }
}
