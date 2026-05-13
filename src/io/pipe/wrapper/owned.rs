use std::time::Duration;

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::io::{
    cache::CacheRead,
    pipe::{Pipe, PipeRead, PipeWrite},
};

use super::{PipeReader, PipeWriter};

pin_project! {
    /// Owned pipe combining separate reader and writer halves.
    ///
    /// The `H` parameter holds the backing resource handle (e.g. a `Child` process).
    /// When the pipe is dropped, the handle is dropped too, cleaning up the resource.
    pub struct OwnedPipe<H, R, W> {
        #[pin]
        reader: PipeReader<R>,
        #[pin]
        writer: PipeWriter<W>,
        handle: H,
    }
}

impl<R, W> OwnedPipe<(), R, W>
where
    PipeReader<R>: PipeRead,
    PipeWriter<W>: PipeWrite,
{
    /// Create a new `OwnedPipe` from reader and writer halves with no backing handle.
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: PipeReader::new(reader),
            writer: PipeWriter::new(writer),
            handle: (),
        }
    }
}

impl<H, R, W> OwnedPipe<H, R, W>
where
    PipeReader<R>: PipeRead,
    PipeWriter<W>: PipeWrite,
{
    /// Create a new `OwnedPipe` from reader, writer, and a backing resource handle.
    pub fn new_with_handle(reader: R, writer: W, handle: H) -> Self {
        Self {
            reader: PipeReader::new(reader),
            writer: PipeWriter::new(writer),
            handle,
        }
    }
}

impl<H, R, W> Pipe for OwnedPipe<H, R, W>
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

impl<H, R, W> AsyncRead for OwnedPipe<H, R, W>
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

impl<H, R, W> CacheRead for OwnedPipe<H, R, W>
where
    PipeReader<R>: CacheRead,
{
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
    }

    fn restore(&mut self, data: &[u8]) {
        self.reader.restore(data);
    }
}

impl<H, R, W> PipeRead for OwnedPipe<H, R, W>
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

impl<H, R, W> AsyncWrite for OwnedPipe<H, R, W>
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

impl<H, R, W> PipeWrite for OwnedPipe<H, R, W>
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
