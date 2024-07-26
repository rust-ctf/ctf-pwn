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

fn timeout_delay(timeout: Option<Duration>) -> Duration {
    match timeout {
        Some(delay) => delay,
        None => PwnTimeout::far_away(),
    }
}

pub trait PipeReadExt: PipeRead {
    fn recv<'a>(&'a mut self) -> Recv<'a, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv::recv(self, 1024 * 4, delay)
    }

    fn recvn<'a>(&'a mut self, size: usize) -> Recv<'a, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv::recv(self, size, delay)
    }

    fn recvall<'a>(&'a mut self) -> RecvAll<'a, Self>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_all::recv_all(self, delay)
    }

    fn recvuntil<'a, D: AsRef<[u8]>>(&'a mut self, delimiter: D) -> RecvUntil<'a, Self, D>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_until::recv_until(self, delimiter, delay)
    }

    fn recvregex<'a>(&'a mut self, pattern: &str) -> Result<RecvRegex<'a, Self>, regex::Error>
    where
        Self: Unpin,
    {
        let delay = timeout_delay(self.read_timeout());
        recv_regex::recv_regex(self, pattern, delay)
    }

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
