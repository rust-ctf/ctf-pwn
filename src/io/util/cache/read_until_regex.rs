use crate::io::AsyncCacheRead;
use pin_project_lite::pin_project;
use regex::bytes::*;
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
    pub struct ReadUntilRegex<'a, R: ?Sized> {
        reader: &'a mut R,
        regex: Regex,
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

pub(crate) fn read_until_regex<'a, R>(
    reader: &'a mut R,
    pattern: &str,
    buf: &'a mut Vec<u8>,
) -> Result<ReadUntilRegex<'a, R>, regex::Error>
where
    R: AsyncCacheRead + ?Sized + Unpin,
{
    let regex = Regex::new(pattern)?;
    Ok(ReadUntilRegex {
        reader,
        regex,
        buf,
        internal_buf: Vec::new(),
        read: 0,
        _pin: PhantomPinned,
    })
}

fn eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "early eof")
}

pub(super) fn read_until_regex_internal<R: AsyncCacheRead + ?Sized>(
    mut reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    regex: &mut Regex,
    buf: &mut Vec<u8>,
    internal_buf: &mut Vec<u8>,
    read: &mut usize,
) -> Poll<io::Result<(usize, usize)>> {
    let mut read_buf = [0u8; 4096];
    let mut data = ReadBuf::new(&mut read_buf);
    loop {
        data.clear();
        ready!(reader.as_mut().poll_reader(cx, &mut data))?;
        let read_len = data.filled().len();
        if read_len == 0 {
            return Err(eof()).into();
        }
        *read += read_len;
        internal_buf.extend_from_slice(data.filled());

        match regex.find(&internal_buf) {
            Some(m) => {
                let drain_index = m.end();
                buf.extend_from_slice(&internal_buf[..drain_index]);
                let restore_data = &internal_buf[drain_index..];
                reader.restore(restore_data);
                *read -= restore_data.len();
                return Poll::Ready(Ok((buf.len(), m.len())));
            }
            None => {}
        }
    }
}

impl<R: AsyncCacheRead + ?Sized + Unpin> Future for ReadUntilRegex<'_, R> {
    type Output = io::Result<(usize, usize)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        read_until_regex_internal(
            Pin::new(*me.reader),
            cx,
            me.regex,
            me.buf,
            me.internal_buf,
            me.read,
        )
    }
}
