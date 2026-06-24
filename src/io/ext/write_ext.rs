//! Extension trait for async writing.

use core::pin::Pin;

use crate::io::error::Error;
use crate::io::write::Write;

impl<T: Write + Unpin> WriteExt for T {}

/// Ergonomic async write operations.
pub trait WriteExt: Write + Unpin {
    /// Write all bytes in `buf`, looping until complete.
    ///
    /// Returns [`Error::UnexpectedEof`] if a write returns 0 bytes
    /// (indicating the writer can no longer accept data).
    fn write_all(
        &mut self,
        buf: &[u8],
    ) -> impl core::future::Future<Output = Result<(), Error<Self::Error>>> {
        async {
            let mut offset = 0;
            while offset < buf.len() {
                let n =
                    core::future::poll_fn(|cx| {
                        Pin::new(&mut *self).poll_write(cx, &buf[offset..])
                    })
                    .await
                    .map_err(Error::Io)?;
                if n == 0 {
                    return Err(Error::UnexpectedEof);
                }
                offset += n;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::runtime_test::runtime_test;
    use crate::io::test_utils::MockWriter;

    use super::*;

    runtime_test!(write_all_writes_everything, {
        let mut w = MockWriter::new();
        w.write_all(b"hello world").await.unwrap();
        assert_eq!(w.written, b"hello world");
    });

    runtime_test!(write_all_empty_is_noop, {
        let mut w = MockWriter::new();
        w.write_all(b"").await.unwrap();
        assert!(w.written.is_empty());
    });
}
