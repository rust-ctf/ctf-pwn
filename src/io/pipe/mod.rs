pub mod ansi;
mod convert;
mod error;
mod interactive;
mod read;
mod write;

pub use convert::*;
pub use error::*;
pub use interactive::*;
pub use read::*;

use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
pub use write::*;

use crate::io::PayloadAction;
use pin_project_lite::pin_project;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use super::cache::*;

pin_project! {
    /// An `AsyncRead`er which applies a timeout to read operations.
    #[derive(Debug)]
    pub struct Pipe<R,W> {
        #[pin]
        reader: CacheReader<R>,
        #[pin]
        writer: W,
        timeout: Duration,
        block_size: usize,
    }
}

impl<R: AsyncRead, W> PipeRead for Pipe<R, W> {
    fn get_timeout(&self) -> Duration {
        self.timeout
    }

    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    fn get_block_size(&self) -> usize {
        self.block_size
    }

    fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }
}

impl<R, W: AsyncWrite> PipeWrite for Pipe<R, W> {}

impl<R, W> Pipe<R, W>
where
    Self: PipeRead + PipeWrite,
{
    async fn payload<T: PayloadAction>(&mut self, payload: T) -> Result<T::ReturnType, PipeError>
    where
        Self: Unpin,
    {
        payload.execute(self).await
    }
}

impl<R: AsyncRead, W> AsyncCacheRead for Pipe<R, W> {
    fn poll_reader(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().reader.poll_reader(cx, buf)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.project().reader.consume(amt)
    }

    fn restore(self: Pin<&mut Self>, data: &[u8]) {
        self.project().reader.restore(data)
    }
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);
    const DEFAULT_BLOCK_SIZE: usize = 4096;

    pub fn new(reader: R, writer: W) -> Pipe<R, W> {
        Pipe {
            reader: CacheReader::new(reader), //CacheReader::new(timeout_reader),
            writer: writer,
            block_size: Self::DEFAULT_BLOCK_SIZE,
            timeout: Self::DEFAULT_TIMEOUT,
        }
    }
}

impl<R: AsyncRead, W> AsyncRead for Pipe<R, W> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.project();
        this.reader.poll_read(cx, buf)
    }
}

impl<R, W: AsyncWrite> AsyncWrite for Pipe<R, W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, Error>> {
        let this = self.project();
        this.writer.poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Error>> {
        let this = self.project();
        this.writer.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Error>> {
        let this = self.project();
        this.writer.poll_shutdown(cx)
    }
}
