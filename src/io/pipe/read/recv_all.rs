use crate::io::pipe::PipeRead;
use crate::io::timeout::*;
use pin_project_lite::pin_project;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

use super::RecvResult;

pub(crate) fn recv_all<'a, R>(reader: &'a mut R, timeout: impl Into<PwnTimeout>) -> RecvAll<'a, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    let buf = Vec::new();
    RecvAll {
        delay: None,
        callback: timeout.into(),
        reader,
        len: 0,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    pub struct RecvAll<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: Vec<u8>,
        len: usize,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for RecvAll<'_, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    type Output = Result<RecvResult, IOTimeoutError>;

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
                let res: &[u8] = me.buf.as_ref();
                return Poll::Ready(Ok(res.into()));
            }
        }

        let delay = me
            .delay
            .get_or_insert_with(|| Box::pin(me.callback.timeout()));

        return match delay.as_mut().poll(cx) {
            Poll::Pending => std::task::Poll::Pending,
            Poll::Ready(()) => {
                let res: &[u8] = me.buf.as_ref();
                return Poll::Ready(Ok(res.into()));
            }
        };
    }
}
