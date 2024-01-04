mod error;
mod interactive;
mod read;
mod write;
pub mod ansi;

pub use error::*;
pub use interactive::*;
pub use read::*;
pub(crate) use ansi::*;

use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
pub use write::*;

use pin_project_lite::pin_project;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use super::{cache::*, timeout::*};

pin_project! {
    /// An `AsyncRead`er which applies a timeout to read operations.
    #[derive(Debug)]
    pub struct Pipe<R,W> {
        #[pin]
        reader: CacheReader<TimeoutReader<R>>,
        #[pin]
        writer: TimeoutWriter<W>,
    }
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

    pub fn new(reader: R, writer: W) -> Pipe<R, W> {
        let mut timeout_reader = TimeoutReader::new(reader);
        let mut timeout_writer = TimeoutWriter::new(writer);

        timeout_reader.set_read_timeout(Some(Self::DEFAULT_TIMEOUT));
        timeout_writer.set_read_timeout(Some(Self::DEFAULT_TIMEOUT));

        Pipe {
            reader: CacheReader::new(timeout_reader), //CacheReader::new(timeout_reader),
            writer: timeout_writer,
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
impl<R, W> HasCache for Pipe<R, W> {
    fn cache_clear(&mut self) {
        self.reader.cache_clear()
    }

    fn cache_insert(&mut self, data: &[u8]) {
        self.reader.cache_insert(data)
    }

    fn has_cache(&self) -> bool {
        self.reader.has_cache()
    }
}

impl<R, W> HasTimeout for Pipe<R, W> {
    fn read_timeout(&self) -> Option<Duration> {
        self.reader.read_timeout()
    }

    fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.reader.set_read_timeout(timeout)
    }

    fn write_timeout(&self) -> Option<Duration> {
        self.writer.write_timeout()
    }

    fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.writer.set_write_timeout(timeout)
    }
}
