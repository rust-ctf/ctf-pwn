mod read_timeout;

use tokio::io::AsyncRead;

use super::PwnTimeout;

impl<R: AsyncRead + ?Sized> TimeoutReadExt for R {}

pub trait TimeoutReadExt: AsyncRead {
    fn read_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: impl Into<PwnTimeout>,
    ) -> read_timeout::ReadTimeout<'a, Self>
    where
        Self: Unpin + Sized,
    {
        read_timeout::read_timeout(self, buf, timeout)
    }
}
