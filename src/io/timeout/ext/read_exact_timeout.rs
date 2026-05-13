use crate::{io::timeout::*, timeout_ready};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn read_exact_timeout<'a, R>(
    reader: &'a mut R,
    buf: &'a mut [u8],
    timeout: impl Into<PwnTimeout>,
) -> ReadExactTimeout<'a, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    ReadExactTimeout {
        delay: None,
        callback: timeout.into(),
        reader,
        buf: ReadBuf::new(buf),
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that reads exactly enough bytes to fill a buffer, with a timeout.
    pub struct ReadExactTimeout<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: ReadBuf<'a>,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for ReadExactTimeout<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = Result<usize, IOTimeoutError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        loop {
            // if our buffer is empty, then we need to read some data to continue.
            let rem = me.buf.remaining();
            if rem != 0 {
                timeout_ready!(
                    Pin::new(&mut *me.reader).poll_read(cx, me.buf),
                    me.delay,
                    me.callback,
                    cx
                )?;
                if me.buf.remaining() == rem {
                    return Err(IOTimeoutError::UnexpectedEof).into();
                }
            } else {
                return Poll::Ready(Ok(me.buf.capacity()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::io::{
        test::{AsyncTestReader, TestAction},
        timeout::{IOTimeoutError, TimeoutReadExt},
    };

    #[tokio::test]
    async fn read_exact_timeout_reads_exact_bytes() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(b"abcdef".to_vec())]);
        let mut buf = [0u8; 4];
        let n = reader
            .read_exact_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_exact_timeout should succeed");
        assert_eq!(n, 4);
        assert_eq!(&buf, b"abcd");
    }

    #[tokio::test]
    async fn read_exact_timeout_across_chunks() {
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(b"ab".to_vec()),
            TestAction::Data(b"cd".to_vec()),
        ]);
        let mut buf = [0u8; 4];
        let n = reader
            .read_exact_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_exact_timeout should succeed");
        assert_eq!(n, 4);
        assert_eq!(&buf, b"abcd");
    }

    #[tokio::test]
    async fn read_exact_timeout_eof_before_filled() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(b"ab".to_vec())]);
        let mut buf = [0u8; 10];
        let err = reader
            .read_exact_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect_err("should error on short read");
        assert!(matches!(err, IOTimeoutError::UnexpectedEof));
    }

    #[tokio::test]
    async fn read_exact_timeout_times_out() {
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(b"ab".to_vec()),
            TestAction::Sleep(Duration::from_secs(10)),
        ]);
        let mut buf = [0u8; 10];
        let err = reader
            .read_exact_timeout(&mut buf, Duration::from_millis(50))
            .await
            .expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }
}
