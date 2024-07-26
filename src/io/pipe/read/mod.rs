mod recv;
mod recv_all;
mod recv_until;
mod result;
pub use recv::*;
pub use recv_all::*;
pub use recv_until::*;
pub use result::*;

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
}
