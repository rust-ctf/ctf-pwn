use crate::{io::timeout::*, timeout_ready};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn read_timeout<'a, R>(
    reader: &'a mut R,
    buf: &'a mut [u8],
    timeout: impl Into<PwnTimeout>,
) -> ReadTimeout<'a, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    ReadTimeout {
        delay: None,
        callback: timeout.into(),
        reader,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that reads into a byte slice with a timeout.
    pub struct ReadTimeout<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: &'a mut [u8],
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for ReadTimeout<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = Result<usize, IOTimeoutError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        let mut buf = ReadBuf::new(me.buf);

        timeout_ready!(
            Pin::new(&mut *me.reader).poll_read(cx, &mut buf),
            me.delay,
            me.callback,
            cx
        )?;

        Poll::Ready(Ok(buf.filled().len()))
    }
}
