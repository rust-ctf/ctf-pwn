use crate::io::util::timeout::{eof, get_deadline, timeout};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::Instant;

/// A future which can be used to easily read bytes until timeout or buf is fully filled
pub(crate) fn read_exact_timeout<'a, A>(
    reader: &'a mut A,
    buf: &'a mut [u8],
    timeout: Duration,
    throw_on_timeout: bool,
) -> ReadExactTimeout<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    let deadline = get_deadline(timeout);
    ReadExactTimeout {
        reader,
        buf: ReadBuf::new(buf),
        deadline,
        _pin: PhantomPinned,
        throw_on_timeout,
    }
}

pin_project! {
    /// Creates a future which will read exactly enough bytes to fill `buf`,
    /// stops if Timeout,
    /// returning an error if EOF is hit sooner.
    ///
    /// On success the number of bytes is returned
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadExactTimeout<'a, A: ?Sized> {
        reader: &'a mut A,
        buf: ReadBuf<'a>,
        deadline: Instant,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
        throw_on_timeout: bool,
    }
}

impl<A> Future for ReadExactTimeout<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let me = self.project();

        loop {
            if *me.deadline < Instant::now() {
                if *me.throw_on_timeout {
                    return Poll::Ready(Err(timeout().into()));
                }
                return Poll::Ready(Ok(me.buf.filled().len()));
            }

            // if our buffer is empty, then we need to read some data to continue.
            let rem = me.buf.remaining();
            if rem != 0 {
                match Pin::new(&mut *me.reader).poll_read(cx, me.buf) {
                    Poll::Ready(Ok(_)) => {}
                    Poll::Ready(Err(e)) if e.kind() == ErrorKind::TimedOut => {
                        continue;
                    }
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(Err(e.into()));
                    }
                    Poll::Pending => continue,
                };
                if me.buf.remaining() == rem {
                    return Err(eof()).into();
                }
            } else {
                return Poll::Ready(Ok(me.buf.capacity()));
            }
        }
    }
}
