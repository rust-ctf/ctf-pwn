use std::{marker::PhantomPinned, time::Duration};

use pin_project_lite::pin_project;
use tokio::io::AsyncRead;

use crate::io::{
    cache::{CacheRead, CacheReader},
    pipe::PipeRead,
};

type Reader<R> = CacheReader<R>;

pin_project! {
    pub struct PipeReader<R> {
        #[pin]
        reader: Reader<R>,
        timeout: Option<Duration>,
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> AsyncRead for PipeReader<R>
where
    Reader<R>: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let me = self.project();
        me.reader.poll_read(cx, buf)
    }
}

impl<R> CacheRead for PipeReader<R>
where
    Reader<R>: CacheRead,
{
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

    fn restore(&mut self, data: &[u8]) {
        self.reader.restore(data)
    }
}

impl<R> PipeRead for PipeReader<R>
where
    Reader<R>: AsyncRead + CacheRead,
{
    fn read_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }
}
