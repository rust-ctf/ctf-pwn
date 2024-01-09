use crate::io::util::cache::read_until::{read_until, ReadUntil};
use crate::io::AsyncCacheRead;

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
}

impl<R: AsyncCacheRead + ?Sized> AsyncCacheReadExt for R {}
