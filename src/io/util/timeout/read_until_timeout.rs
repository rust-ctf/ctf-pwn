use crate::io::ReadUntil;
use crate::io::{get_deadline, read_until, AsyncCacheRead};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::time::Instant;
use crate::io::util::timeout::timeout;

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
    let read_until = read_until(reader, delimiter, buf);
    let deadline = get_deadline(timeout);
    ReadUntilTimeout {
        read_until,
        deadline,
        _pin: PhantomPinned,
    }
}

impl<R: AsyncCacheRead + ?Sized + Unpin, D: AsRef<[u8]>> Future for ReadUntilTimeout<'_, R, D> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        if *me.deadline < Instant::now() {
            return Err(timeout()).into();
        }

        me.read_until.poll(cx)
    }
}
