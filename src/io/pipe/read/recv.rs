use super::RecvResult;
use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn recv<R>(
    reader: &mut R,
    size: usize,
    timeout: impl Into<PwnTimeout>,
) -> Recv<'_, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    let buf: Vec<u8> = vec![0u8; size];
    Recv {
        delay: None,
        callback: timeout.into(),
        reader,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that receives up to a fixed number of bytes.
    pub struct Recv<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: Vec<u8>,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for Recv<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = Result<RecvResult, PipeError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        let mut buf = ReadBuf::new(me.buf);

        //Fill data we have on start
        loop {
            if buf.remaining() == 0 {
                return Poll::Ready(Ok(buf.filled().into()));
            }

            let before = buf.filled().len();
            match Pin::new(&mut *me.reader).poll_read(cx, &mut buf) {
                Poll::Ready(_) => {
                    if buf.filled().len() == before {
                        // EOF: no new bytes were read
                        break;
                    }
                }
                Poll::Pending => break,
            }
        }

        // If we have any data return
        if !buf.filled().is_empty() {
            return Poll::Ready(Ok(buf.filled().into()));
        }

        let delay = me
            .delay
            .get_or_insert_with(|| Box::pin(me.callback.timeout()));

        match delay.as_mut().poll(cx) {
            Poll::Pending => std::task::Poll::Pending,
            Poll::Ready(()) => std::task::Poll::Ready(Err(PipeError::Timeout)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::io::{
        pipe::{PipeError, PipeRead, PipeReadExt, PipeReader},
        test::{AsyncTestReader, TestAction},
    };

    fn test_pipe(actions: &[TestAction]) -> PipeReader<AsyncTestReader> {
        let mut pipe = PipeReader::new(AsyncTestReader::new(actions));
        pipe.set_read_timeout(Some(Duration::from_millis(200)));
        pipe
    }

    fn test_pipe_with_timeout(actions: &[TestAction], ms: u64) -> PipeReader<AsyncTestReader> {
        let mut pipe = test_pipe(actions);
        pipe.set_read_timeout(Some(Duration::from_millis(ms)));
        pipe
    }

    #[tokio::test]
    async fn recv_returns_available_data() {
        let mut pipe = test_pipe(&[TestAction::Data(b"hello".to_vec())]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(result.as_bytes(), b"hello");
    }

    #[tokio::test]
    async fn recv_returns_partial_data_up_to_4k() {
        let data = vec![0xABu8; 8192];
        let mut pipe = test_pipe(&[TestAction::Data(data)]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(result.as_bytes().len(), 4096);
    }

    #[tokio::test]
    async fn recv_returns_data_after_sleep() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Sleep(Duration::from_millis(50)),
                TestAction::Data(b"delayed".to_vec()),
            ],
            500,
        );
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(result.as_bytes(), b"delayed");
    }

    #[tokio::test]
    async fn recv_timeout_on_no_data() {
        let mut pipe = test_pipe_with_timeout(
            &[TestAction::Sleep(Duration::from_secs(10))],
            50,
        );
        let err = pipe.recv().await.expect_err("recv should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recv_eof_with_no_data_times_out() {
        let mut pipe = test_pipe_with_timeout(&[], 50);
        let err = pipe.recv().await.expect_err("recv on immediate EOF should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }
}
