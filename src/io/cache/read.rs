use std::{
    marker::PhantomPinned,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf};

use super::CacheRead;

pin_project! {
    /// An `AsyncRead`er which applies a timeout to read operations.
    #[derive(Debug)]
    pub struct CacheReader<R> {
        #[pin]
        pub(crate) reader: R,
        #[pin]
        pub(crate) cache: Vec<u8>,
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> CacheReader<R> {
    pub fn new(reader: R) -> CacheReader<R> {
        CacheReader {
            reader,
            cache: Vec::new(),
            _pin: PhantomPinned,
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
    fn consume(&mut self, len: usize) {
        self.cache.drain(..len);
    }

    fn restore(&mut self, data: &[u8]) {
        self.cache.extend_from_slice(data)
    }
}
