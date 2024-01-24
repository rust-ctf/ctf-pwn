use crate::io::{get_deadline, AsyncCacheRead};
use crate::io::{read_until_regex, ReadUntilRegex};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::io::util::timeout::timeout;
use tokio::time::Instant;

pin_project! {
    /// The delimiter is included in the resulting vector.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadUntilRegexTimeout<'a, R: ?Sized> {
        #[pin]
        read_until_regex: ReadUntilRegex<'a,R>,
        deadline: Instant,
        #[pin]
        _pin: PhantomPinned,
    }
}

pub(crate) fn read_until_regex_timeout<'a, R>(
    reader: &'a mut R,
    pattern: &str,
    buf: &'a mut Vec<u8>,
    timeout: Duration,
) -> Result<ReadUntilRegexTimeout<'a, R>, regex::Error>
where
    R: AsyncCacheRead + ?Sized + Unpin,
{
    let read_until_regex = read_until_regex(reader, pattern, buf)?;
    let deadline = get_deadline(timeout);
    Ok(ReadUntilRegexTimeout {
        read_until_regex,
        deadline,
        _pin: PhantomPinned,
    })
}

impl<R: AsyncCacheRead + ?Sized + Unpin> Future for ReadUntilRegexTimeout<'_, R> {
    type Output = io::Result<(usize, usize)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        if *me.deadline < Instant::now() {
            return Err(timeout()).into();
        }

        me.read_until_regex.poll(cx)
    }
}
