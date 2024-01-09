use std::task::Poll;
use pin_project_lite::pin_project;
use tokio::io::*;

//TODO: AutoGenerate MergedOutput for up to ten args

pin_project! {
    #[derive(Debug)]
    pub struct MergedAsyncReader<R1, R2> {
        #[pin]
        pub stream0: R1,
        #[pin]
        pub stream1: R2,
    }
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
        let me = self.project();
        match me.stream0.poll_read(cx, buf) {
            Poll::Ready(r) => Poll::Ready(r),
            Poll::Pending => me.stream1.poll_read(cx, buf),
        }
    }
}
