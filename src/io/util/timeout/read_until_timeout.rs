use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use std::time::Duration;
use tokio::io::ReadBuf;
use tokio::time::Instant;
use crate::io::{AsyncCacheRead, get_deadline, read_until};
use crate::io::ReadUntil;

pin_project! {
    /// The delimiter is included in the resulting vector.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadUntilTimeout<'a, R: ?Sized, D:AsRef<[u8]>> {
        #[pin]
        read_until: ReadUntil<'a,R,D>,
        deadline: Instant,
        #[pin]
        _pin: PhantomPinned,
    }
}

pub(crate) fn read_until_timeout<'a, R, D: AsRef<[u8]>>(
    reader: &'a mut R,
    delimiter: D,
    buf: &'a mut Vec<u8>,
    timeout: Duration,
) -> ReadUntilTimeout<'a, R, D>
    where
        R: AsyncCacheRead + ?Sized + Unpin,
{
    let read_until = read_until(reader,delimiter, buf);
    let deadline = get_deadline(timeout);
    ReadUntilTimeout {
        read_until,
        deadline,
        _pin: PhantomPinned,
    }
}

fn timeout() -> io::Error {
    io::Error::new(ErrorKind::TimedOut, "early timeout")
}

impl<R: AsyncCacheRead + ?Sized + Unpin, D:AsRef<[u8]>> Future for ReadUntilTimeout<'_, R, D> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        if *me.deadline < Instant::now()
        {
            return Err(timeout()).into();
        }

        me.read_until.poll(cx)
    }
}
