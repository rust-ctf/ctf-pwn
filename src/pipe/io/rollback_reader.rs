use std::io::{self};
use tokio::io::{AsyncRead, ReadBuf};

pub struct RollbackReader<R> {
    inner: R,
    rollback_buf: Vec<u8>,
}

impl<R: AsyncRead + Unpin> RollbackReader<R> {
    pub fn new(inner: R) -> Self {
        RollbackReader {
            inner,
            rollback_buf: Vec::new(),
        }
    }

    pub fn rollback(&mut self, data: &[u8]) {
        self.rollback_buf.extend_from_slice(data);
    }

    pub fn read_until()
    {

    }
}

impl<R: AsyncRead + Unpin> AsyncRead for RollbackReader<R> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        // If there is data in the rollback buffer, consume from it first.
        if !self.rollback_buf.is_empty() {
            let len = self.rollback_buf.len().max(buf.remaining());
            buf.put_slice(&self.rollback_buf[..len]);
            self.rollback_buf.drain(..len);
            return std::task::Poll::Ready(Ok(()));//len));
        }

        // If rollback buffer is empty, delegate to the inner reader.
         // If rollback buffer is empty, delegate to the inner reader.
         match std::pin::Pin::new(&mut self.inner).poll_read(cx, buf) {
            std::task::Poll::Ready(Ok(())) => std::task::Poll::Ready(Ok(())),//buf.len())),
            std::task::Poll::Ready(Err(e)) => std::task::Poll::Ready(Err(e)),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}