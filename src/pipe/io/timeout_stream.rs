use pin_project_lite::pin_project;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::time::*;

pin_project! {
    #[derive(Debug)]
    pub struct TimeoutSream<T>
    {
        #[pin]
        stream: T,
        #[pin]
        read_delay: Option<Sleep>,
        #[pin]
        write_delay: Option<Sleep>,
        timeout: Duration,
    }
}

impl<T> TimeoutSream<T> {
    pub fn new(stream: T, timeout: Duration) -> Self {
        Self {
            stream,
            read_delay: None,
            write_delay: None,
            timeout,
        }
    }
}

impl<R: AsyncRead> AsyncRead for TimeoutSream<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let mut me = self.project();
        if let Poll::Ready(v) = me.stream.poll_read(cx, buf) {
            return Poll::Ready(v);
        }

        if me.read_delay.is_none() {
            me.read_delay.as_mut().set(Some(sleep(*me.timeout)));
        }

        //todo: Extract method
        let delay = me.read_delay;
        let poll_delay = || -> Poll<io::Result<()>> {
            match delay.as_pin_mut() {
                None => unreachable!(),
                Some(delay) => match delay.poll(cx) {
                    Poll::Ready(()) => Poll::Ready(Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "read operation timed out",
                    ))),
                    Poll::Pending => Poll::Pending,
                },
            }
        };

        //TODO: Add coop budget check

        poll_delay()
    }
}

impl<T: AsyncWrite> AsyncWrite for TimeoutSream<T> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        let mut me = self.project();
        if let Poll::Ready(v) = me.stream.poll_write(cx, buf) {
            return Poll::Ready(v);
        }

        if me.write_delay.is_none() {
            me.write_delay.as_mut().set(Some(sleep(*me.timeout)));
        }

        let delay = me.write_delay;
        let poll_delay = || -> Poll<io::Result<usize>> {
            match delay.as_pin_mut() {
                None => unreachable!(),
                Some(delay) => match delay.poll(cx) {
                    Poll::Ready(()) => Poll::Ready(Err(Error::new(
                        io::ErrorKind::TimedOut,
                        "write operation timed out",
                    ))),
                    Poll::Pending => Poll::Pending,
                },
            }
        };

        //TODO: Add coop budget check

        poll_delay()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let mut me = self.project();
        if let Poll::Ready(v) = me.stream.poll_flush(cx) {
            return Poll::Ready(v);
        }

        if me.write_delay.is_none() {
            me.write_delay.as_mut().set(Some(sleep(*me.timeout)));
        }

        let delay = me.write_delay;
        let poll_delay = || -> Poll<io::Result<()>> {
            match delay.as_pin_mut() {
                None => unreachable!(),
                Some(delay) => match delay.poll(cx) {
                    Poll::Ready(()) => Poll::Ready(Err(Error::new(
                        io::ErrorKind::TimedOut,
                        "write operation timed out",
                    ))),
                    Poll::Pending => Poll::Pending,
                },
            }
        };

        //TODO: Add coop budget check

        poll_delay()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let mut me = self.project();
        if let Poll::Ready(v) = me.stream.poll_shutdown(cx, ) {
            return Poll::Ready(v);
        }

        if me.write_delay.is_none() {
            me.write_delay.as_mut().set(Some(sleep(*me.timeout)));
        }

        let delay = me.write_delay;
        let poll_delay = || -> Poll<io::Result<()>> {
            match delay.as_pin_mut() {
                None => unreachable!(),
                Some(delay) => match delay.poll(cx) {
                    Poll::Ready(()) => Poll::Ready(Err(Error::new(
                        io::ErrorKind::TimedOut,
                        "write operation timed out",
                    ))),
                    Poll::Pending => Poll::Pending,
                },
            }
        };

        //TODO: Add coop budget check

        poll_delay()
    }
}
