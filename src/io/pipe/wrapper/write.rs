use std::time::Duration;

use pin_project_lite::pin_project;
use tokio::io::AsyncWrite;

use crate::io::pipe::PipeWrite;

/// Type alias for the inner writer.
type Writer<W> = W;

pin_project! {
    /// Pipe writer wrapper with timeout support.
    pub struct PipeWriter<W> {
        #[pin]
        writer: Writer<W>,
        timeout: Option<Duration>,
    }
}

impl<W> PipeWriter<W>
where
    Self: PipeWrite,
{
    /// Create a new `PipeWriter` wrapping the given writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            timeout: Some(Self::DEFAULT_WRITE_TIMEOUT),
        }
    }
}

impl<W> AsyncWrite for PipeWriter<W>
where
    Writer<W>: AsyncWrite,
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

impl<W> PipeWrite for PipeWriter<W>
where
    Writer<W>: AsyncWrite,
{
    fn write_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }
}
