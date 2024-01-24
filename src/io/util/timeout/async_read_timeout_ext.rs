use crate::io::util::timeout::read_exact_timeout::{read_exact_timeout, ReadExactTimeout};
use std::time::Duration;
use tokio::io::AsyncRead;
use crate::io::{read_timeout, ReadTimeout};
use crate::io::util::timeout::read_to_end_timeout::{read_to_end_timeout, ReadToEndTimeout};

pub trait AsyncReadTimeoutExt: AsyncRead {
    fn read_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: Duration,
    ) -> ReadTimeout<'a, Self>
        where
            Self: Unpin,
    {
        read_timeout(self, buf, timeout)
    }

    fn read_fill_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: Duration,
    ) -> ReadExactTimeout<'a, Self>
        where
            Self: Unpin,
    {
        read_exact_timeout(self, buf, timeout, false)
    }

    fn read_exact_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: Duration,
    ) -> ReadExactTimeout<'a, Self>
    where
        Self: Unpin,
    {
        read_exact_timeout(self, buf, timeout, true)
    }



    fn read_to_end_timeout<'a>(&'a mut self, buf: &'a mut Vec<u8>, timeout: Duration, throw_on_timeout: bool)
    -> ReadToEndTimeout<'a, Self>
    where
        Self: Unpin,
    {
        read_to_end_timeout(self, buf, timeout, throw_on_timeout)
    }
}

impl<R: AsyncRead + ?Sized> AsyncReadTimeoutExt for R {}
