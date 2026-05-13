use crate::{io::timeout::*, timeout_ready};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, marker::PhantomPinned};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn read_timeout<'a, R>(
    reader: &'a mut R,
    buf: &'a mut [u8],
    timeout: impl Into<PwnTimeout>,
) -> ReadTimeout<'a, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    ReadTimeout {
        delay: None,
        callback: timeout.into(),
        reader,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that reads into a byte slice with a timeout.
    pub struct ReadTimeout<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: &'a mut [u8],
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R> Future for ReadTimeout<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = Result<usize, IOTimeoutError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut me = self.project();

        let mut buf = ReadBuf::new(me.buf);

        timeout_ready!(
            Pin::new(&mut *me.reader).poll_read(cx, &mut buf),
            me.delay,
            me.callback,
            cx
        )?;

        Poll::Ready(Ok(buf.filled().len()))
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
    async fn read_timeout_reads_available_data() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(b"hello".to_vec())]);
        let mut buf = [0u8; 32];
        let n = reader
            .read_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_timeout should succeed");
        assert_eq!(&buf[..n], b"hello");
    }

    #[tokio::test]
    async fn read_timeout_returns_zero_on_eof() {
        let mut reader = AsyncTestReader::new(&[]);
        let mut buf = [0u8; 32];
        let n = reader
            .read_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_timeout should succeed");
        assert_eq!(n, 0);
    }

    #[tokio::test]
    async fn read_timeout_times_out() {
        let mut reader = AsyncTestReader::new(&[TestAction::Sleep(Duration::from_secs(10))]);
        let mut buf = [0u8; 32];
        let err = reader
            .read_timeout(&mut buf, Duration::from_millis(50))
            .await
            .expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }

    #[tokio::test]
    async fn read_timeout_partial_fill() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(b"ab".to_vec())]);
        let mut buf = [0u8; 10];
        let n = reader
            .read_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_timeout should succeed");
        assert_eq!(n, 2);
        assert_eq!(&buf[..n], b"ab");
    }
}
