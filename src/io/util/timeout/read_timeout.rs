use crate::io::util::timeout::get_deadline;
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
pub(crate) fn read_timeout<'a, A>(
    reader: &'a mut A,
    buf: &'a mut [u8],
    timeout: Duration,
) -> ReadTimeout<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    let deadline = get_deadline(timeout);
    ReadTimeout {
        reader,
        buf: ReadBuf::new(buf),
        deadline,
        _pin: PhantomPinned,
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
    pub struct ReadTimeout<'a, A: ?Sized> {
        reader: &'a mut A,
        buf: ReadBuf<'a>,
        deadline: Instant,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
    }
}

fn timeout() -> io::Error {
    io::Error::new(ErrorKind::TimedOut, "early timeout")
}

impl<A> Future for ReadTimeout<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let me = self.project();

        loop {
            if *me.deadline < Instant::now() {
                return Poll::Ready(Err(timeout()));
            }

            match ready!(Pin::new(&mut *me.reader).poll_read(cx, me.buf)) {
                Ok(_) => {
                    return Poll::Ready(Ok(me.buf.capacity()));
                }
                Err(e) if e.kind() == ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    return Poll::Ready(Err(e.into()));
                }
            };
        }
    }
}
