use crate::io::timeout::*;
use pin_project_lite::pin_project;
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::Poll;
use tokio::io::{AsyncRead, ReadBuf};

/// A future which can be used to easily read bytes until timeout or buf is fully filled
pub(crate) fn read_timeout<'a, A: AsyncRead>(
    reader: &'a mut A,
    buf: &'a mut [u8],
    timeout: impl Into<PwnTimeout>,
) -> ReadTimeout<'a, A> {
    let buf = ReadBuf::new(buf);
    ReadTimeout {
        delay: None,
        callback: timeout.into(),
        reader,
        buf,
    }
}

pin_project! {
    pub struct ReadTimeout<'a, T: ?Sized> {
        reader: &'a mut T,
        buf: ReadBuf<'a>,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
    }
}

impl<'a, T: AsyncRead + Unpin + ?Sized> Future for ReadTimeout<'a, T> {
    type Output = Result<usize, IOTimeoutError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        if let Poll::Ready(result) = Pin::new(&mut *me.reader).poll_read(cx, me.buf) {
            return Poll::Ready(match result {
                Ok(_) => Ok(me.buf.capacity() - me.buf.remaining()),
                Err(err) => Err(err.into()),
            });
        }

        let delay = me
            .delay
            .get_or_insert_with(|| Box::pin(me.callback.timeout()));

        match delay.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => Poll::Ready(Err(IOTimeoutError::Timeout)),
        }
    }
}
