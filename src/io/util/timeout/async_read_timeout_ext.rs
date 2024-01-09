use crate::io::util::timeout::read_exact_timeout::{read_exact_timeout, ReadExactTimeout};
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

pub trait AsyncReadTimeoutExt: AsyncRead {
    fn read_exact_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: Duration,
    ) -> ReadExactTimeout<'a, Self>
    where
        Self: Unpin,
    {
        read_exact_timeout(self, buf, timeout)
    }

    fn read_to_end_timeout<'a>(&'a mut self, buf: &'a mut Vec<u8>, timeout: Duration)
    where
        Self: Unpin,
    {
        todo!()
    }

    fn read_to_string_timeout<'a>(&'a mut self, dst: &'a mut String, timeout: Duration)
    where
        Self: Unpin,
    {
        todo!()
    }
}

impl<R: AsyncRead + ?Sized> AsyncReadTimeoutExt for R {}
