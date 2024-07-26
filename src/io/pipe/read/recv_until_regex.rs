use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use regex::bytes::*;
use std::marker::Unpin;
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
                me.reader.restore(&me.buf);
                return Poll::Ready(Err(PipeError::UnexpectedEof));
            }

            me.buf.append(&mut buf.filled().to_vec());

            //TODO: Optimize to only use last part of buff (of pattern len) when pattern matching
            match me.rx.captures(&me.buf) {
                Some(capture) => {
                    let full_match = capture
                        .get(0)
                        .expect("Failed to get regex match value")
                        .as_bytes();
                    let delim_len = full_match.len();
                    let offset = kmp::kmp_find(full_match, &me.buf)
                        .expect("Failed to get regex match offset");

                    let drain_index = offset + delim_len;
                    let restore_data = &me.buf[drain_index..];
                    me.reader.restore(restore_data);
                    let res: &[u8] = &me.buf[..drain_index];
                    return Poll::Ready(Ok(res.into()));
                }
                None => {}
            }
        }

        let delay = me
            .delay
            .get_or_insert_with(|| Box::pin(me.callback.timeout()));

        return match delay.as_mut().poll(cx) {
            Poll::Pending => std::task::Poll::Pending,
            Poll::Ready(()) => {
                me.reader.restore(&me.buf);
                return Poll::Ready(Err(PipeError::Timeout));
            }
        };
    }
}
