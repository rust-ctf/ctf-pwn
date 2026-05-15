use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use regex::bytes::*;
use std::pin::Pin;
use std::str::FromStr;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

use super::RecvRegexResult;

pub(crate) fn recv_regex<'a, R>(
    reader: &'a mut R,
    pattern: &str,
    timeout: impl Into<PwnTimeout>,
) -> Result<RecvRegex<'a, R>, regex::Error>
where
    R: PipeRead + Unpin + ?Sized,
{
    let rx = Regex::from_str(pattern)?;
    let buf = Vec::new();
    Ok(RecvRegex {
        delay: None,
        callback: timeout.into(),
        rx,
        reader,
        buf,
        _pin: PhantomPinned,
    })
}

pin_project! {
    /// Future that receives data until a regex pattern matches.
    pub struct RecvRegex<'a, R: ?Sized> {
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

impl<R> Future for RecvRegex<'_, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    type Output = Result<RecvRegexResult, PipeError>;

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

                return Poll::Ready(Ok(capture.into()));
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
    async fn recvregex_matches_pattern() {
        let mut pipe = test_pipe(&[TestAction::Data(b"value=42 rest".to_vec())]);
        let result = pipe
            .recvregex(r"value=(\d+)")
            .expect("valid regex")
            .await
            .expect("recvregex should succeed");
        assert_eq!(result.full_match().as_bytes(), b"value=42");
        assert_eq!(result.groups().len(), 2);
        assert_eq!(result.groups()[1].as_bytes(), b"42");
    }

    #[tokio::test]
    async fn recvregex_across_chunks() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"val".to_vec()),
                TestAction::Data(b"ue=99 done".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvregex(r"value=(\d+)")
            .expect("valid regex")
            .await
            .expect("recvregex should succeed");
        assert_eq!(result.full_match().as_bytes(), b"value=99");
    }

    #[tokio::test]
    async fn recvregex_invalid_pattern() {
        let mut pipe = test_pipe(&[]);
        let err = pipe.recvregex(r"(unclosed");
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn recvregex_timeout_when_no_match() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"no match here".to_vec()),
                TestAction::Sleep(Duration::from_secs(10)),
            ],
            50,
        );
        let err = pipe
            .recvregex(r"value=\d+")
            .expect("valid regex")
            .await
            .expect_err("should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recvregex_eof_before_match() {
        let mut pipe = test_pipe(&[TestAction::Data(b"no match".to_vec())]);
        let err = pipe
            .recvregex(r"value=\d+")
            .expect("valid regex")
            .await
            .expect_err("should error on EOF");
        assert!(matches!(err, PipeError::UnexpectedEof));
    }

    #[tokio::test]
    async fn recvregex_delayed_match_arrives_in_time() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"pre".to_vec()),
                TestAction::Sleep(Duration::from_millis(30)),
                TestAction::Data(b"value=77 post".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvregex(r"value=(\d+)")
            .expect("valid regex")
            .await
            .expect("match should arrive in time");
        assert_eq!(result.full_match().as_bytes(), b"value=77");
    }

    #[tokio::test]
    async fn recvregex_delayed_match_arrives_too_late() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"pre".to_vec()),
                TestAction::Sleep(Duration::from_millis(200)),
                TestAction::Data(b"value=77 post".to_vec()),
            ],
            50,
        );
        let err = pipe
            .recvregex(r"value=(\d+)")
            .expect("valid regex")
            .await
            .expect_err("should timeout");
        assert!(matches!(err, PipeError::Timeout));
    }

    #[tokio::test]
    async fn recvregex_multiple_delays_before_match() {
        let mut pipe = test_pipe_with_timeout(
            &[
                TestAction::Data(b"x".to_vec()),
                TestAction::Sleep(Duration::from_millis(15)),
                TestAction::Data(b"y".to_vec()),
                TestAction::Sleep(Duration::from_millis(15)),
                TestAction::Data(b"value=88 end".to_vec()),
            ],
            500,
        );
        let result = pipe
            .recvregex(r"value=(\d+)")
            .expect("valid regex")
            .await
            .expect("cumulative delays within timeout");
        assert_eq!(result.full_match().as_bytes(), b"value=88");
        assert_eq!(result.groups()[1].as_bytes(), b"88");
    }
}
