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
