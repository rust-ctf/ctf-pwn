use super::RecvResult;
use crate::io::pipe::PipeRead;
use crate::{io::timeout::*, timeout_ready};
use pin_project_lite::pin_project;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn recv<'a, R>(
    reader: &'a mut R,
    size: usize,
    timeout: impl Into<PwnTimeout>,
) -> Recv<'a, R>
where
    R: PipeRead + Unpin + ?Sized,
{
    Recv {
        delay: None,
        callback: timeout.into(),
        reader,
        size,
        _pin: PhantomPinned,
    }
}

pin_project! {
    pub struct Recv<'a, R: ?Sized> {
        reader: &'a mut R,
        size: usize,
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
    type Output = Result<RecvResult, IOTimeoutError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        let mut result: Vec<u8> = vec![0u8; *me.size];
        let mut buf = ReadBuf::new(&mut result);

        //Fill data we have on start
        loop {
            if buf.remaining() == 0 {
                return Poll::Ready(Ok(buf.filled().into()));
            }

            match Pin::new(&mut *me.reader).poll_read(cx, &mut buf) {
                Poll::Ready(_) => continue,
                Poll::Pending => break,
            }
        }

        // If we have any data return
        if !buf.filled().is_empty() {
            return Poll::Ready(Ok(buf.filled().into()));
        }

        timeout_ready!(
            Pin::new(&mut *me.reader).poll_read(cx, &mut buf),
            me.delay,
            me.callback,
            cx
        )?;

        Poll::Ready(Ok(buf.filled().into()))
    }
}
