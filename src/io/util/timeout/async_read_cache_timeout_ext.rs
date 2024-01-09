use crate::io::util::timeout::read_until_timeout::{read_until_timeout, ReadUntilTimeout};
use crate::io::{AsyncCacheRead, AsyncReadTimeoutExt};
use std::time::Duration;
use tokio::io::AsyncRead;

pub trait AsyncReadCacheTimeoutExt: AsyncCacheRead {
    fn read_until_timeout<'a, T: AsRef<[u8]>>(
        &'a mut self,
        delim: T,
        buf: &'a mut Vec<u8>,
        timeout: Duration,
    ) -> ReadUntilTimeout<'a, Self, T>
    where
        Self: Unpin,
    {
        read_until_timeout(self, delim, buf, timeout)
    }

    fn read_line_timeout<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
        timeout: Duration,
    ) -> ReadUntilTimeout<'a, Self, &'static [u8]>
    where
        Self: Unpin,
    {
        read_until_timeout(self, b"\n", buf, timeout)
    }

    fn read_line_crlf_timeout<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
        timeout: Duration,
    ) -> ReadUntilTimeout<'a, Self, &'static [u8]>
    where
        Self: Unpin,
    {
        read_until_timeout(self, b"\r\n", buf, timeout)
    }
}

impl<R: AsyncCacheRead + ?Sized> AsyncReadCacheTimeoutExt for R {}
