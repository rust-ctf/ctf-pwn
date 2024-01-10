use crate::io::util::cache::read_until::{read_until, ReadUntil};
use crate::io::{read_until_regex, AsyncCacheRead, ReadUntilRegex};

pub trait AsyncCacheReadExt: AsyncCacheRead {
    fn read_until<'a, T: AsRef<[u8]>>(
        &'a mut self,
        delim: T,
        buf: &'a mut Vec<u8>,
    ) -> ReadUntil<'a, Self, T>
    where
        Self: Unpin,
    {
        read_until(self, delim, buf)
    }

    fn read_line<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadUntil<'a, Self, &'static [u8]>
    where
        Self: Unpin,
    {
        read_until(self, b"\n", buf)
    }

    fn read_line_crlf<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadUntil<'a, Self, &'static [u8]>
    where
        Self: Unpin,
    {
        read_until(self, b"\r\n", buf)
    }

    async fn read_until_regex<'a>(
        &'a mut self,
        pattern: &str,
        buf: &'a mut Vec<u8>,
    ) -> Result<ReadUntilRegex<'a, Self>, regex::Error>
    where
        Self: Unpin,
    {
        read_until_regex(self, pattern, buf)
    }
}

impl<R: AsyncCacheRead + ?Sized> AsyncCacheReadExt for R {}
