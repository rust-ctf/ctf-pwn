use std::{
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf};

use super::CacheRead;

pin_project! {
    /// Async reader that buffers data and supports cache operations.
    #[derive(Debug)]
    pub struct CacheReader<R> {
        #[pin]
        pub(crate) reader: R,
        #[pin]
        pub(crate) cache: Vec<u8>,
    }
}

impl<R> CacheReader<R> {
    /// Create a new `CacheReader` wrapping the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            cache: Vec::new(),
        }
    }
}

impl<R: AsyncRead> AsyncRead for CacheReader<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut this = self.project();
        if !this.cache.is_empty() {
            let remaining = usize::min(buf.remaining(), this.cache.len());
            buf.put_slice(&this.cache[..remaining]);
            this.cache.drain(..remaining);
            return Poll::Ready(Ok(()));
        }
        this.reader.poll_read(cx, buf)
    }
}

impl<R: AsyncRead> CacheRead for CacheReader<R> {
    fn consume(&mut self, amt: usize) {
        self.cache.drain(..amt);
    }

    fn restore(&mut self, data: &[u8]) {
        self.cache.extend_from_slice(data);
    }
}
