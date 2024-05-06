use crate::io::util::timeout::{get_deadline, timeout};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::{BufReader, ErrorKind};
use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::Instant;

pub(crate) fn read_to_end_timeout<'a, A>(
    reader: &'a mut A,
    buf: &'a mut Vec<u8>,
    timeout: Duration,
    throw_on_timeout: bool,
) -> ReadToEndTimeout<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    let deadline = get_deadline(timeout);
    ReadToEndTimeout {
        reader,
        buf: buf,
        deadline,
        _pin: PhantomPinned,
        throw_on_timeout,
    }
}

pin_project! {
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadToEndTimeout<'a, A: ?Sized> {
        reader: &'a mut A,
        buf: &'a mut Vec<u8>,
        deadline: Instant,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
        throw_on_timeout: bool,
    }
}

impl<A> Future for ReadToEndTimeout<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let me = self.project();
        let mut read_buf = [0u8; 4096];
        let mut data = ReadBuf::new(&mut read_buf);
        loop {
            if *me.deadline < Instant::now() {
                if *me.throw_on_timeout {
                    return Poll::Ready(Err(timeout().into()));
                }
                break;
            }

            match Pin::new(&mut *me.reader).poll_read(cx, &mut data) {
                Poll::Ready(Ok(_)) => {}
                Poll::Ready(Err(e)) if e.kind() == ErrorKind::TimedOut => {
                    continue;
                }
                Poll::Ready(Err(e)) => {
                    return Poll::Ready(Err(e.into()));
                }
                Poll::Pending => continue
            };

            //eof
            if data.filled().len() == 0 {
                break;
            }
            me.buf.extend_from_slice(data.filled());
            data.clear();
        }

        Poll::Ready(Ok(me.buf.len()))
    }
}
