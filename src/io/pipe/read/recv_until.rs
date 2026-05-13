use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

use super::RecvResult;

pub(crate) fn recv_until<R, D>(
    reader: &mut R,
    delimiter: D,
    timeout: impl Into<PwnTimeout>,
) -> RecvUntil<'_, R, D>
where
    R: PipeRead + Unpin + ?Sized,
    D: AsRef<[u8]>,
{
    let buf = Vec::new();
    RecvUntil {
        delay: None,
        callback: timeout.into(),
        delimiter,
        reader,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that receives data until a byte delimiter is found.
    pub struct RecvUntil<'a, R: ?Sized, D: AsRef<[u8]>> {
        reader: &'a mut R,
        buf: Vec<u8>,
        delimiter: D,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R, D> Future for RecvUntil<'_, R, D>
where
    R: PipeRead + Unpin + ?Sized,
    D: AsRef<[u8]>,
{
    type Output = Result<RecvResult, PipeError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();
        let delim_len = me.delimiter.as_ref().len();

        let mut buf = [0u8; 1024];

        loop {
            let mut buf = ReadBuf::new(&mut buf);

            match Pin::new(&mut *me.reader).poll_read(cx, &mut buf) {
                Poll::Ready(_) => {}
                Poll::Pending => break,
            }

            //EOF
            if buf.filled().is_empty() {
                me.reader.restore(me.buf);
                return Poll::Ready(Err(PipeError::UnexpectedEof));
            }

            me.buf.append(&mut buf.filled().to_vec());

            //TODO: Optimize to only use last part of buff (of pattern len) when pattern matching
            if let Some(offset) = kmp::kmp_find(me.delimiter.as_ref(), me.buf) {
                let drain_index = offset + delim_len;
                let restore_data = &me.buf[drain_index..];
                me.reader.restore(restore_data);
                let res: &[u8] = &me.buf[..drain_index];
                return Poll::Ready(Ok(res.into()));
            }
        }

        let delay = me
            .delay
            .get_or_insert_with(|| Box::pin(me.callback.timeout()));

        match delay.as_mut().poll(cx) {
            Poll::Pending => std::task::Poll::Pending,
            Poll::Ready(()) => {
                me.reader.restore(me.buf);
                Poll::Ready(Err(PipeError::Timeout))
            }
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
    async fn recvuntil_finds_delimiter() {
        let mut pipe = test_pipe(&[TestAction::Data(b"hello\nworld".to_vec())]);
        let result = pipe.recvuntil(b"\n").await.expect("recvuntil should succeed");
        assert_eq!(result.as_bytes(), b"hello\n");
    }

    #[tokio::test]
    async fn recvuntil_multi_byte_delimiter() {
        let mut pipe = test_pipe(&[TestAction::Data(b"foo::bar::baz".to_vec())]);
        let result = pipe.recvuntil(b"::").await.expect("recvuntil should succeed");
        assert_eq!(result.as_bytes(), b"foo::");
    }

    #[tokio::test]
    async fn recvuntil_delimiter_across_chunks() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"hel".to_vec()),
                TestAction::Data(b"lo\nworld".to_vec()),
            ],
            500,
        );
        let result = pipe.recvuntil(b"\n").await.expect("recvuntil should succeed");
        assert_eq!(result.as_bytes(), b"hello\n");
    }

    #[tokio::test]
    async fn recvuntil_timeout_when_delimiter_not_found() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"no delimiter here".to_vec()),
                TestAction::Sleep(Duration::from_secs(10)),
            ],
            50,
        );
        let err = pipe.recvuntil(b"\n").await.expect_err("should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recvuntil_eof_before_delimiter() {
        let mut pipe = test_pipe(&[TestAction::Data(b"no newline".to_vec())]);
        let err = pipe
            .recvuntil(b"\n")
            .await
            .expect_err("should error on EOF");
        assert!(matches!(err, PipeError::UnexpectedEof));
    }

    #[tokio::test]
    async fn recvuntil_delimiter_at_start() {
        let mut pipe = test_pipe(&[TestAction::Data(b"\nhello".to_vec())]);
        let result = pipe.recvuntil(b"\n").await.expect("recvuntil should succeed");
        assert_eq!(result.as_bytes(), b"\n");
    }

    #[tokio::test]
    async fn recvuntil_restores_data_after_delimiter() {
        let mut pipe = test_pipe(&[TestAction::Data(b"first\nsecond\n".to_vec())]);
        let r1 = pipe.recvuntil(b"\n").await.expect("first recvuntil");
        assert_eq!(r1.as_bytes(), b"first\n");
        let r2 = pipe.recvuntil(b"\n").await.expect("second recvuntil");
        assert_eq!(r2.as_bytes(), b"second\n");
    }
}
