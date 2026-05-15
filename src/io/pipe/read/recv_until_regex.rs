use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use regex::bytes::*;
use std::pin::Pin;
use std::str::FromStr;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

use super::RecvResult;

pub(crate) fn recv_until_regex<'a, R>(
    reader: &'a mut R,
    pattern: &str,
    timeout: impl Into<PwnTimeout>,
) -> Result<RecvUntilRegex<'a, R>, regex::Error>
where
    R: PipeRead + Unpin + ?Sized,
{
    let rx = Regex::from_str(pattern)?;
    let buf = Vec::new();
    Ok(RecvUntilRegex {
        delay: None,
        callback: timeout.into(),
        rx,
        reader,
        buf,
        _pin: PhantomPinned,
    })
}

pin_project! {
    /// Future that receives data until a regex pattern matches, returning up to the match.
    pub struct RecvUntilRegex<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: Vec<u8>,
        rx: Regex,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for RecvUntilRegex<'_, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    type Output = Result<RecvResult, PipeError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();
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
            if let Some(capture) = me.rx.captures(me.buf) {
                let full_match = capture
                    .get(0)
                    .expect("Failed to get regex match value")
                    .as_bytes();
                let delim_len = full_match.len();
                let offset = kmp::kmp_find(full_match, me.buf)
                    .expect("Failed to get regex match offset");

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
    async fn recvuntilregex_returns_data_up_to_match() {
        let mut pipe = test_pipe(&[TestAction::Data(b"prefix>>>marker<<<suffix".to_vec())]);
        let result = pipe
            .recvuntilregex(r">>>.*?<<<")
            .expect("valid regex")
            .await
            .expect("recvuntilregex should succeed");
        assert_eq!(result.as_bytes(), b"prefix>>>marker<<<");
    }

    #[tokio::test]
    async fn recvuntilregex_across_chunks() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"data".to_vec()),
                TestAction::Data(b"END rest".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect("recvuntilregex should succeed");
        assert_eq!(result.as_bytes(), b"dataEND");
    }

    #[tokio::test]
    async fn recvuntilregex_timeout_when_no_match() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"nothing".to_vec()),
                TestAction::Sleep(Duration::from_secs(10)),
            ],
            50,
        );
        let err = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect_err("should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recvuntilregex_eof_before_match() {
        let mut pipe = test_pipe(&[TestAction::Data(b"nothing".to_vec())]);
        let err = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect_err("should error on EOF");
        assert!(matches!(err, PipeError::UnexpectedEof));
    }

    #[tokio::test]
    async fn recvuntilregex_restores_data_after_match() {
        let mut pipe = test_pipe(&[TestAction::Data(b"line1\nline2\n".to_vec())]);
        let r1 = pipe
            .recvuntilregex(r"\n")
            .expect("valid regex")
            .await
            .expect("first recv");
        assert_eq!(r1.as_bytes(), b"line1\n");
        let r2 = pipe
            .recvuntilregex(r"\n")
            .expect("valid regex")
            .await
            .expect("second recv");
        assert_eq!(r2.as_bytes(), b"line2\n");
    }

    #[tokio::test]
    async fn recvuntilregex_delayed_match_arrives_in_time() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"data".to_vec()),
                TestAction::Sleep(Duration::from_millis(30)),
                TestAction::Data(b"END rest".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect("match should arrive in time");
        assert_eq!(result.as_bytes(), b"dataEND");
    }

    #[tokio::test]
    async fn recvuntilregex_delayed_match_arrives_too_late() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"data".to_vec()),
                TestAction::Sleep(Duration::from_millis(200)),
                TestAction::Data(b"END rest".to_vec()),
            ],
            50,
        );
        let err = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect_err("should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recvuntilregex_cumulative_delays_within_timeout() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"a".to_vec()),
                TestAction::Sleep(Duration::from_millis(15)),
                TestAction::Data(b"b".to_vec()),
                TestAction::Sleep(Duration::from_millis(15)),
                TestAction::Data(b"cEND rest".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect("cumulative delays within timeout");
        assert_eq!(result.as_bytes(), b"abcEND");
    }

    #[tokio::test]
    async fn recvuntilregex_cumulative_delays_exceed_timeout() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"a".to_vec()),
                TestAction::Sleep(Duration::from_millis(30)),
                TestAction::Data(b"b".to_vec()),
                TestAction::Sleep(Duration::from_millis(30)),
                TestAction::Data(b"cEND".to_vec()),
            ],
            50,
        );
        let err = pipe
            .recvuntilregex(r"END")
            .expect("valid regex")
            .await
            .expect_err("cumulative delays exceed timeout");
        assert!(matches!(err, PipeError::Timeout));
    }
}
