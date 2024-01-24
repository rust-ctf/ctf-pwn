use crate::io::AsyncCacheRead;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use tokio::io::ReadBuf;

pin_project! {
    /// The delimiter is included in the resulting vector.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadUntil<'a, R: ?Sized, D:AsRef<[u8]>> {
        reader: &'a mut R,
        delimiter: D,
        buf: &'a mut Vec<u8>,
        internal_buf: Vec<u8>,
        // The number of bytes appended to buf. This can be less than buf.len() if
        // the buffer was not empty when the operation was started.
        read: usize,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
    }
}

pub(crate) fn read_until<'a, R, D: AsRef<[u8]>>(
    reader: &'a mut R,
    delimiter: D,
    buf: &'a mut Vec<u8>,
) -> ReadUntil<'a, R, D>
where
    R: AsyncCacheRead + ?Sized + Unpin,
{
    ReadUntil {
        reader,
        delimiter,
        buf,
        internal_buf: Vec::new(),
        read: 0,
        _pin: PhantomPinned,
    }
}

fn eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "early eof")
}

pub(super) fn read_until_internal<R: AsyncCacheRead + ?Sized, D: AsRef<[u8]>>(
    mut reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    delimiter: D,
    buf: &mut Vec<u8>,
    internal_buf: &mut Vec<u8>,
    read: &mut usize,
) -> Poll<io::Result<usize>> {
    let delim_len = delimiter.as_ref().len();
    if delim_len == 0 {
        return Poll::Ready(Ok(0));
    }

    let mut read_buf = [0u8; 4096];
    let mut data = ReadBuf::new(&mut read_buf);
    loop {
        data.clear();
        match ready!(reader.as_mut().poll_reader(cx, &mut data))
        {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => { return Poll::Ready(Err(e.into())); }
        }
        let read_len = data.filled().len();
        if read_len == 0 {
            return Err(eof()).into();
        }
        *read += read_len;
        internal_buf.extend_from_slice(data.filled());

        match kmp::kmp_find(delimiter.as_ref(), &internal_buf) {
            Some(offset) => {
                let drain_index = offset + delim_len;
                buf.extend_from_slice(&internal_buf[..drain_index]);
                let restore_data = &internal_buf[drain_index..];
                reader.restore(restore_data);
                *read -= restore_data.len();
                return Poll::Ready(Ok(buf.len()));
            }
            None => {
                if internal_buf.len() >= delim_len {
                    let drain_range = 0..internal_buf.len() - delim_len;
                    buf.extend_from_slice(&internal_buf[drain_range.clone()]);
                    internal_buf.drain(drain_range);
                }
            }
        }
    }
}

impl<R: AsyncCacheRead + ?Sized + Unpin, D: AsRef<[u8]>> Future for ReadUntil<'_, R, D> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        read_until_internal(
            Pin::new(*me.reader),
            cx,
            me.delimiter,
            me.buf,
            me.internal_buf,
            me.read,
        )
    }
}
