use std::task::Poll;
use tokio::io::*;

//TODO: AutoGenerate MergedOutput for up to ten args

#[derive(Debug)]
pub struct MergedAsyncReader<R1, R2> {
    pub stream0: R1,
    pub stream1: R2,
}

impl<R1, R2> MergedAsyncReader<R1, R2> {
    pub fn new(stream0: R1, stream1: R2) -> MergedAsyncReader<R1, R2> {
        MergedAsyncReader { stream0, stream1 }
    }
}

impl<R1: AsyncRead + Unpin, R2: AsyncRead + Unpin> AsyncRead for MergedAsyncReader<R1, R2> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let stream0_bytes = std::pin::Pin::new(&mut self.stream0).poll_read(cx, buf);
        match stream0_bytes {
            Poll::Ready(Ok(r)) if buf.capacity() == buf.filled().len() => Poll::Ready(Ok(r)),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            _ => std::pin::Pin::new(&mut self.stream1).poll_read(cx, buf),
        }
    }
}
