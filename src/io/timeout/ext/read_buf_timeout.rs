use crate::{io::timeout::*, timeout_ready};
use bytes::BufMut;
use pin_project_lite::pin_project;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, task::Context};
use tokio::io::{AsyncRead, ReadBuf};

pub(crate) fn read_buf_timeout<'a, R, B>(
    reader: &'a mut R,
    buf: &'a mut B,
    timeout: impl Into<PwnTimeout>,
) -> ReadBufTimeout<'a, R, B>
where
    R: AsyncRead + Unpin + ?Sized,
    B: BufMut + ?Sized,
{
    ReadBufTimeout {
        delay: None,
        callback: timeout.into(),
        reader,
        buf,
        _pin: PhantomPinned,
    }
}

pin_project! {
    /// Future that reads into a `BufMut` with a timeout.
    pub struct ReadBufTimeout<'a, R: ?Sized, B: ?Sized> {
        reader: &'a mut R,
        buf: &'a mut B,
        #[pin]
        delay: Option<BoxSleep>,
        callback: PwnTimeout, // callback to create the timer
        #[pin]
        _pin: PhantomPinned,
    }
}

impl<R, B> Future for ReadBufTimeout<'_, R, B>
where
    R: AsyncRead + Unpin + ?Sized,
    B: BufMut + ?Sized,
{
    type Output = Result<usize, IOTimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<usize, IOTimeoutError>> {
        use std::mem::MaybeUninit;

        let mut me = self.project();

        if !me.buf.has_remaining_mut() {
            return Poll::Ready(Ok(0));
        }

        let n = {
            let dst = me.buf.chunk_mut();
            // SAFETY: `UninitSlice` and `[MaybeUninit<u8>]` have the same memory layout.
            let dst = unsafe { &mut *(std::ptr::from_mut(dst) as *mut [MaybeUninit<u8>]) };
            let mut buf = ReadBuf::uninit(dst);
            let ptr = buf.filled().as_ptr();
            timeout_ready!(
                Pin::new(&mut *me.reader).poll_read(cx, &mut buf),
                me.delay,
                me.callback,
                cx
            )?;

            // Ensure the pointer does not change from under us
            assert_eq!(ptr, buf.filled().as_ptr());
            buf.filled().len()
        };

        // Safety: This is guaranteed to be the number of initialized (and read)
        // bytes due to the invariants provided by `ReadBuf::filled`.
        unsafe {
            me.buf.advance_mut(n);
        }

        Poll::Ready(Ok(n))
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
    async fn read_buf_timeout_reads_into_vec() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(b"world".to_vec())]);
        let mut buf = Vec::with_capacity(32);
        let n = reader
            .read_buf_timeout(&mut buf, Duration::from_secs(1))
            .await
            .expect("read_buf_timeout should succeed");
        assert_eq!(n, 5);
        assert_eq!(&buf[..], b"world");
    }

    #[tokio::test]
    async fn read_buf_timeout_times_out() {
        let mut reader = AsyncTestReader::new(&[TestAction::Sleep(Duration::from_secs(10))]);
        let mut buf = Vec::with_capacity(32);
        let err = reader
            .read_buf_timeout(&mut buf, Duration::from_millis(50))
            .await
            .expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }
}
