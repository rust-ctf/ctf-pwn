//! Shared test utilities for I/O tests.
//!
//! Provides programmable mock readers and writers that simulate
//! chunked data delivery, timed delays, and EOF.

use alloc::collections::VecDeque;
use core::convert::Infallible;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use crate::io::read::Read;
use crate::io::timer::{Timer, Sleep, TimerProvider};
use crate::io::write::Write;

/// An action a [`MockReader`] will perform.
#[derive(Debug, Clone)]
pub(crate) enum ReadAction {
    /// Return this data chunk (may be partial if buf is smaller).
    Data(Vec<u8>),
    /// Sleep for the given duration before advancing to the next action.
    Sleep(Duration),
}

/// Programmable mock reader driven by a queue of [`ReadAction`] values.
///
/// Uses [`Timer`] for sleep actions. When the queue is empty,
/// returns EOF (`Ok(0)`).
pub(crate) struct MockReader {
    actions: VecDeque<ReadAction>,
    active_sleep: Option<Sleep>,
}

impl Unpin for MockReader {}

impl MockReader {
    pub(crate) fn new(actions: &[ReadAction]) -> Self {
        Self {
            actions: actions.iter().cloned().collect(),
            active_sleep: None,
        }
    }
}

impl Read for MockReader {
    type Error = Infallible;

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Infallible>> {
        if let Some(sleep) = &mut self.active_sleep {
            match Pin::new(sleep).poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(()) => {
                    self.active_sleep = None;
                }
            }
        }

        let Some(action) = self.actions.front_mut() else {
            return Poll::Ready(Ok(0));
        };

        match action {
            ReadAction::Data(data) => {
                let n = data.len().min(buf.len());
                buf[..n].copy_from_slice(&data[..n]);
                if n == data.len() {
                    self.actions.pop_front();
                } else {
                    data.drain(..n);
                }
                Poll::Ready(Ok(n))
            }
            ReadAction::Sleep(duration) => {
                let duration = *duration;
                self.actions.pop_front();
                self.active_sleep = Some(Timer::sleep(duration));
                // Re-enter — the active_sleep check at the top will poll it
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

/// Programmable mock writer that records written data.
///
/// Accepts all data immediately.
pub(crate) struct MockWriter {
    pub(crate) written: Vec<u8>,
}

impl Unpin for MockWriter {}

impl MockWriter {
    /// Create a writer that accepts everything immediately.
    pub(crate) fn new() -> Self {
        Self {
            written: Vec::new(),
        }
    }
}

impl Write for MockWriter {
    type Error = Infallible;

    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Infallible>> {
        self.written.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Infallible>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Infallible>> {
        Poll::Ready(Ok(()))
    }
}

/// Mock bidirectional I/O (implements both Read and Write).
pub(crate) struct MockStream {
    pub(crate) reader: MockReader,
    pub(crate) writer: MockWriter,
}

impl Unpin for MockStream {}

impl MockStream {
    pub(crate) fn new(read_actions: &[ReadAction]) -> Self {
        Self {
            reader: MockReader::new(read_actions),
            writer: MockWriter::new(),
        }
    }
}

impl Read for MockStream {
    type Error = Infallible;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Infallible>> {
        let reader = &mut self.get_mut().reader;
        Pin::new(reader).poll_read(cx, buf)
    }
}

impl Write for MockStream {
    type Error = Infallible;

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Infallible>> {
        let writer = &mut self.get_mut().writer;
        Pin::new(writer).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Infallible>> {
        let writer = &mut self.get_mut().writer;
        Pin::new(writer).poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Infallible>> {
        let writer = &mut self.get_mut().writer;
        Pin::new(writer).poll_shutdown(cx)
    }
}

#[cfg(all(test, not(feature = "runtime-embassy")))]
mod tests {
    use super::*;

    async fn assert_sleep_then_data() {
        let mut reader = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(50)),
            ReadAction::Data(b"after_sleep".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let start = std::time::Instant::now();
        let result =
            core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await;
        let elapsed = start.elapsed();

        assert_eq!(result.unwrap(), 11);
        assert_eq!(&buf[..11], b"after_sleep");
        assert!(
            elapsed >= Duration::from_millis(40),
            "sleep should have delayed at least 40ms, got {elapsed:?}"
        );
    }

    async fn assert_sleep_duration_is_accurate() {
        let mut reader = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(100)),
            ReadAction::Data(b"x".to_vec()),
        ]);
        let mut buf = [0u8; 1];

        let start = std::time::Instant::now();
        let _ =
            core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await;
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(80),
            "expected ~100ms sleep, got {elapsed:?}"
        );
        assert!(
            elapsed < Duration::from_millis(300),
            "sleep took too long: {elapsed:?}"
        );
    }

    async fn assert_multiple_sleeps_accumulate_time() {
        let mut reader = MockReader::new(&[
            ReadAction::Data(b"a".to_vec()),
            ReadAction::Sleep(Duration::from_millis(30)),
            ReadAction::Data(b"b".to_vec()),
            ReadAction::Sleep(Duration::from_millis(30)),
            ReadAction::Data(b"c".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let start = std::time::Instant::now();
        for expected in [b"a".as_slice(), b"b", b"c"] {
            let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
                .await
                .unwrap();
            assert_eq!(&buf[..n], expected);
        }
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(50),
            "two 30ms sleeps should take at least 50ms, got {elapsed:?}"
        );
    }

    async fn assert_no_sleep_is_instant() {
        let mut reader = MockReader::new(&[ReadAction::Data(b"fast".to_vec())]);
        let mut buf = [0u8; 64];

        let start = std::time::Instant::now();
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        let elapsed = start.elapsed();

        assert_eq!(&buf[..n], b"fast");
        assert!(
            elapsed < Duration::from_millis(10),
            "data-only read should be near-instant, got {elapsed:?}"
        );
    }

    async fn assert_data_chunks_and_eof() {
        let mut reader = MockReader::new(&[
            ReadAction::Data(b"hello".to_vec()),
            ReadAction::Data(b"world".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"hello");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"world");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(n, 0);
    }

    async fn assert_partial_when_buf_small() {
        let mut reader = MockReader::new(&[ReadAction::Data(b"abcdef".to_vec())]);
        let mut buf = [0u8; 3];

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"abc");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"def");
    }

    #[cfg(feature = "runtime-tokio")]
    mod tokio_tests {
        use super::*;

        #[tokio::test]
        async fn sleep_then_data() { assert_sleep_then_data().await; }
        #[tokio::test]
        async fn sleep_duration_is_accurate() { assert_sleep_duration_is_accurate().await; }
        #[tokio::test]
        async fn multiple_sleeps_accumulate() { assert_multiple_sleeps_accumulate_time().await; }
        #[tokio::test]
        async fn no_sleep_is_instant() { assert_no_sleep_is_instant().await; }
        #[tokio::test]
        async fn data_chunks_and_eof() { assert_data_chunks_and_eof().await; }
        #[tokio::test]
        async fn partial_when_buf_small() { assert_partial_when_buf_small().await; }
    }

    #[cfg(feature = "runtime-async-std")]
    mod async_std_tests {
        use super::*;

        #[async_std::test]
        async fn sleep_then_data() { assert_sleep_then_data().await; }
        #[async_std::test]
        async fn sleep_duration_is_accurate() { assert_sleep_duration_is_accurate().await; }
        #[async_std::test]
        async fn multiple_sleeps_accumulate() { assert_multiple_sleeps_accumulate_time().await; }
        #[async_std::test]
        async fn no_sleep_is_instant() { assert_no_sleep_is_instant().await; }
        #[async_std::test]
        async fn data_chunks_and_eof() { assert_data_chunks_and_eof().await; }
        #[async_std::test]
        async fn partial_when_buf_small() { assert_partial_when_buf_small().await; }
    }

    #[cfg(feature = "runtime-smol")]
    mod smol_tests {
        use super::*;

        #[test]
        fn sleep_then_data() { smol::block_on(assert_sleep_then_data()); }
        #[test]
        fn sleep_duration_is_accurate() { smol::block_on(assert_sleep_duration_is_accurate()); }
        #[test]
        fn multiple_sleeps_accumulate() { smol::block_on(assert_multiple_sleeps_accumulate_time()); }
        #[test]
        fn no_sleep_is_instant() { smol::block_on(assert_no_sleep_is_instant()); }
        #[test]
        fn data_chunks_and_eof() { smol::block_on(assert_data_chunks_and_eof()); }
        #[test]
        fn partial_when_buf_small() { smol::block_on(assert_partial_when_buf_small()); }
    }
}
