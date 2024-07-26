use std::time::Duration;

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::io::{
    cache::CacheRead,
    pipe::{Pipe, PipeRead, PipeWrite},
};

use super::{PipeReader, PipeWriter};

pin_project! {
    pub struct OwnedPipe<R, W> {
        #[pin]
        reader: PipeReader<R>,
        #[pin]
        writer: PipeWriter<W>,
    }
}

impl<R, W> OwnedPipe<R, W>
where
    PipeReader<R>: PipeRead,
    PipeWriter<W>: PipeWrite,
{
    pub fn new(reader: R, writer: W) -> OwnedPipe<R, W> {
        OwnedPipe {
            reader: PipeReader::new(reader),
            writer: PipeWriter::new(writer),
        }
    }
}

impl<R, W> Pipe for OwnedPipe<R, W>
where
    PipeReader<R>: PipeRead,
    PipeWriter<W>: PipeWrite,
{
    fn split(&mut self) -> (&mut impl PipeRead, &mut impl PipeWrite) {
        (&mut self.reader, &mut self.writer)
    }

    fn reader(&mut self) -> &mut impl PipeRead {
        &mut self.reader
    }

    fn writer(&mut self) -> &mut impl PipeWrite {
        &mut self.writer
    }
}

impl<R, W> AsyncRead for OwnedPipe<R, W>
where
    PipeReader<R>: AsyncRead,
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

impl<R, W> CacheRead for OwnedPipe<R, W>
where
    PipeReader<R>: CacheRead,
{
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

    fn restore(&mut self, data: &[u8]) {
        self.reader.restore(data)
    }
}

impl<R, W> PipeRead for OwnedPipe<R, W>
where
    PipeReader<R>: PipeRead,
{
    fn read_timeout(&self) -> Option<Duration> {
        self.reader.read_timeout()
    }

    fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.reader.set_read_timeout(timeout);
    }
}

impl<R, W> AsyncWrite for OwnedPipe<R, W>
where
    PipeWriter<W>: AsyncWrite,
{
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let me = self.project();
        me.writer.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let me = self.project();
        me.writer.poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let me = self.project();
        me.writer.poll_shutdown(cx)
    }
}

impl<R, W> PipeWrite for OwnedPipe<R, W>
where
    PipeWriter<W>: PipeWrite,
{
    fn write_timeout(&self) -> Option<Duration> {
        self.writer.write_timeout()
    }

    fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.writer.set_write_timeout(timeout);
    }
}
