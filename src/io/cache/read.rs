use crate::io::cache::CacheReader;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::{AsyncRead, ReadBuf};

impl<R: AsyncRead> AsyncRead for CacheReader<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
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
