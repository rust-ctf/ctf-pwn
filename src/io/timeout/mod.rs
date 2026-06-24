//! Timeout wrappers for async I/O operations.
//!
//! Provides [`TimeoutReader`], [`TimeoutWriter`], and [`TimeoutStream`] that
//! add deadline-aware polling to [`Read`], [`Write`], or combined types.

mod reader;
mod stream;
mod writer;

pub use reader::TimeoutReader;
pub use stream::TimeoutStream;
pub use writer::TimeoutWriter;
#[cfg(test)]
mod tests {
    use core::convert::Infallible;
    use core::pin::Pin;
    use core::time::Duration;

    use crate::io::error::Error;
    use crate::io::read::Read;
    use crate::io::runtime_test::runtime_test;
    use crate::io::test_utils::{MockReader, MockStream, MockWriter, ReadAction};
    use crate::io::write::Write;

    use super::*;

    runtime_test!(reader_succeeds_within_timeout, {
        let inner = MockReader::new(&[ReadAction::Data(b"hello".to_vec())]);
        let mut reader = TimeoutReader::new(inner, Some(Duration::from_millis(100)));

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"hello");
    });

    runtime_test!(reader_times_out_on_slow_inner, {
        let inner = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(200)),
            ReadAction::Data(b"too late".to_vec()),
        ]);
        let mut reader = TimeoutReader::new(inner, Some(Duration::from_millis(50)));

        let mut buf = [0u8; 64];
        let result = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await;
        assert!(matches!(result, Err(Error::Timeout)));
    });

    runtime_test!(reader_no_timeout_waits_indefinitely, {
        let inner = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(30)),
            ReadAction::Data(b"arrived".to_vec()),
        ]);
        let mut reader = TimeoutReader::new(inner, None);

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"arrived");
    });

    runtime_test!(reader_deadline_resets_after_success, {
        let inner = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(30)),
            ReadAction::Data(b"first".to_vec()),
            ReadAction::Sleep(Duration::from_millis(30)),
            ReadAction::Data(b"second".to_vec()),
        ]);
        let mut reader = TimeoutReader::new(inner, Some(Duration::from_millis(50)));

        let mut buf = [0u8; 64];
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"first");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap();
        assert_eq!(&buf[..n], b"second");
    });

    runtime_test!(reader_set_timeout_changes_behavior, {
        let inner = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(100)),
            ReadAction::Data(b"data".to_vec()),
        ]);
        let mut reader = TimeoutReader::new(inner, Some(Duration::from_millis(200)));

        reader.set_timeout(Some(Duration::from_millis(30)));

        let mut buf = [0u8; 64];
        let result = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await;
        assert!(matches!(result, Err(Error::Timeout)));
    });

    runtime_test!(writer_succeeds_within_timeout, {
        let inner = MockWriter::new();
        let mut writer = TimeoutWriter::new(inner, Some(Duration::from_millis(100)));

        let n = core::future::poll_fn(|cx| Pin::new(&mut writer).poll_write(cx, b"hello"))
            .await
            .unwrap();
        assert_eq!(n, 5);
        assert_eq!(writer.inner().written, b"hello");
    });

    runtime_test!(stream_read_times_out, {
        

        let inner = MockStream::new(&[
            ReadAction::Sleep(Duration::from_millis(200)),
            ReadAction::Data(b"late".to_vec()),
        ]);
        let mut stream = TimeoutStream::new(inner, Some(Duration::from_millis(50)));

        let mut buf = [0u8; 64];
        let result: Result<usize, Error<Infallible>> =
            core::future::poll_fn(|cx| Pin::new(&mut stream).poll_read(cx, &mut buf)).await;
        assert!(matches!(result, Err(Error::Timeout)));
    });

    runtime_test!(stream_read_succeeds_write_succeeds, {
        

        let inner = MockStream::new(&[ReadAction::Data(b"input".to_vec())]);
        let mut stream = TimeoutStream::new(inner, Some(Duration::from_millis(100)));

        let mut buf = [0u8; 64];
        let n: usize =
            core::future::poll_fn(|cx| Pin::new(&mut stream).poll_read(cx, &mut buf))
                .await
                .unwrap();
        assert_eq!(&buf[..n], b"input");

        let n: usize =
            core::future::poll_fn(|cx| Pin::new(&mut stream).poll_write(cx, b"output"))
                .await
                .unwrap();
        assert_eq!(n, 6);
    });

    runtime_test!(stream_separate_read_write_timeouts, {
        

        let inner = MockStream::new(&[
            ReadAction::Sleep(Duration::from_millis(200)),
            ReadAction::Data(b"data".to_vec()),
        ]);
        let mut stream = TimeoutStream::new(inner, None);
        stream.set_read_timeout(Some(Duration::from_millis(50)));
        stream.set_write_timeout(Some(Duration::from_millis(500)));

        let mut buf = [0u8; 64];
        let result: Result<usize, Error<Infallible>> =
            core::future::poll_fn(|cx| Pin::new(&mut stream).poll_read(cx, &mut buf)).await;
        assert!(matches!(result, Err(Error::Timeout)));
    });
}
