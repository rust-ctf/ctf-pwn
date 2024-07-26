use crate::io::pipe::{PipeError, PipeRead};
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

use super::RecvResult;

pub(crate) fn recv_until<'a, R, D>(
    reader: &'a mut R,
    delimiter: D,
    timeout: impl Into<PwnTimeout>,
) -> RecvUntil<'a, R, D>
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
                let res: &[u8] = me.buf.as_ref();
                return Poll::Ready(Ok(res.into()));
            }

            me.buf.append(&mut buf.filled().to_vec());

            //TODO: Optimize to only use last part of buff (of pattern len) when pattern matching
            match kmp::kmp_find(me.delimiter.as_ref(), &me.buf) {
                Some(offset) => {
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
