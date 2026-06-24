//! Extension trait for async buffered reading.

use core::pin::Pin;

use crate::io::buffered::Buffered;
use crate::io::error::Error;

impl<T: Buffered + Unpin> ReadExt for T {}

macro_rules! read_int_methods {
    ($($(#[doc = $doc:expr])* $name:ident -> $ty:ty, $size:expr, $conv:ident;)*) => {
        $(
            $(#[doc = $doc])*
            fn $name(&mut self) -> impl core::future::Future<Output = Result<$ty, Error<Self::Error>>> {
                async {
                    let mut buf = [0u8; $size];
                    self.read_exact(&mut buf).await?;
                    Ok(<$ty>::$conv(buf))
                }
            }
        )*
    };
}

/// Ergonomic async read operations on any [`Buffered`] type.
pub trait ReadExt: Buffered + Unpin {
    /// Read exactly `buf.len()` bytes.
    ///
    /// Returns [`Error::UnexpectedEof`] if the stream ends before the buffer is filled.
    fn read_exact(
        &mut self,
        buf: &mut [u8],
    ) -> impl core::future::Future<Output = Result<(), Error<Self::Error>>> {
        async {
            let mut offset = 0;
            while offset < buf.len() {
                let n = core::future::poll_fn(|cx| {
                    Pin::new(&mut *self).poll_read(cx, &mut buf[offset..])
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

    /// Read all data until EOF.
    fn read_all(
        &mut self,
    ) -> impl core::future::Future<Output = Result<Vec<u8>, Error<Self::Error>>> {
        async {
            loop {
                let n = core::future::poll_fn(|cx| Pin::new(&mut *self).poll_fill(cx))
                    .await
                    .map_err(Error::Io)?;
                if n == 0 {
                    let data = self.buffer().to_vec();
                    self.consume(data.len());
                    return Ok(data);
                }
            }
        }
    }

    /// Read until `delimiter` is found in the stream.
    ///
    /// Returns data including the delimiter. Data after the delimiter
    /// remains in the buffer for subsequent reads.
    fn read_until(
        &mut self,
        delimiter: &[u8],
    ) -> impl core::future::Future<Output = Result<Vec<u8>, Error<Self::Error>>> {
        async {
            let delim_len = delimiter.len();
            loop {
                if let Some(offset) = kmp::kmp_find(delimiter, self.buffer()) {
                    let end = offset + delim_len;
                    let result = self.buffer()[..end].to_vec();
                    self.consume(end);
                    return Ok(result);
                }

                let n = core::future::poll_fn(|cx| Pin::new(&mut *self).poll_fill(cx))
                    .await
                    .map_err(Error::Io)?;
                if n == 0 {
                    return Err(Error::UnexpectedEof);
                }
            }
        }
    }

    /// Read a `u8`.
    fn read_u8(
        &mut self,
    ) -> impl core::future::Future<Output = Result<u8, Error<Self::Error>>> {
        async {
            let mut buf = [0u8; 1];
            self.read_exact(&mut buf).await?;
            Ok(buf[0])
        }
    }

    /// Read an `i8`.
    fn read_i8(
        &mut self,
    ) -> impl core::future::Future<Output = Result<i8, Error<Self::Error>>> {
        async {
            let mut buf = [0u8; 1];
            self.read_exact(&mut buf).await?;
            Ok(i8::from_be_bytes(buf))
        }
    }

    read_int_methods! {
        /// Read a big-endian `u16`.
        read_u16_be -> u16, 2, from_be_bytes;
        /// Read a little-endian `u16`.
        read_u16_le -> u16, 2, from_le_bytes;
        /// Read a big-endian `i16`.
        read_i16_be -> i16, 2, from_be_bytes;
        /// Read a little-endian `i16`.
        read_i16_le -> i16, 2, from_le_bytes;
        /// Read a big-endian `u32`.
        read_u32_be -> u32, 4, from_be_bytes;
        /// Read a little-endian `u32`.
        read_u32_le -> u32, 4, from_le_bytes;
        /// Read a big-endian `i32`.
        read_i32_be -> i32, 4, from_be_bytes;
        /// Read a little-endian `i32`.
        read_i32_le -> i32, 4, from_le_bytes;
        /// Read a big-endian `u64`.
        read_u64_be -> u64, 8, from_be_bytes;
        /// Read a little-endian `u64`.
        read_u64_le -> u64, 8, from_le_bytes;
        /// Read a big-endian `i64`.
        read_i64_be -> i64, 8, from_be_bytes;
        /// Read a little-endian `i64`.
        read_i64_le -> i64, 8, from_le_bytes;
        /// Read a big-endian `u128`.
        read_u128_be -> u128, 16, from_be_bytes;
        /// Read a little-endian `u128`.
        read_u128_le -> u128, 16, from_le_bytes;
        /// Read a big-endian `i128`.
        read_i128_be -> i128, 16, from_be_bytes;
        /// Read a little-endian `i128`.
        read_i128_le -> i128, 16, from_le_bytes;
        /// Read a big-endian `f32`.
        read_f32_be -> f32, 4, from_be_bytes;
        /// Read a little-endian `f32`.
        read_f32_le -> f32, 4, from_le_bytes;
        /// Read a big-endian `f64`.
        read_f64_be -> f64, 8, from_be_bytes;
        /// Read a little-endian `f64`.
        read_f64_le -> f64, 8, from_le_bytes;
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use crate::io::buffered::BufferedReader;
    use crate::io::runtime_test::runtime_test;
    use crate::io::test_utils::{MockReader, ReadAction};

    use super::*;

    fn buffered(actions: &[ReadAction]) -> BufferedReader<MockReader> {
        BufferedReader::new(MockReader::new(actions))
    }

    fn data(d: &[u8]) -> ReadAction {
        ReadAction::Data(d.to_vec())
    }

    runtime_test!(read_exact_fills_buffer, {
        let mut r = buffered(&[data(b"hello")]);
        let mut buf = [0u8; 5];
        r.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");
    });

    runtime_test!(read_exact_across_chunks, {
        let mut r = buffered(&[data(b"hel"), data(b"lo")]);
        let mut buf = [0u8; 5];
        r.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");
    });

    runtime_test!(read_exact_across_sleep, {
        let mut r = buffered(&[
            data(b"ab"),
            ReadAction::Sleep(Duration::from_millis(10)),
            data(b"cd"),
        ]);
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"abcd");
    });

    runtime_test!(read_exact_eof_before_filled, {
        let mut r = buffered(&[data(b"ab")]);
        let mut buf = [0u8; 10];
        let err = r.read_exact(&mut buf).await.unwrap_err();
        assert!(matches!(err, Error::UnexpectedEof));
    });

    runtime_test!(read_all_collects_until_eof, {
        let mut r = buffered(&[data(b"hello"), data(b" world")]);
        let result = r.read_all().await.unwrap();
        assert_eq!(result, b"hello world");
    });

    runtime_test!(read_all_empty_on_immediate_eof, {
        let mut r = buffered(&[]);
        let result = r.read_all().await.unwrap();
        assert!(result.is_empty());
    });

    runtime_test!(read_all_with_sleeps, {
        let mut r = buffered(&[
            data(b"a"),
            ReadAction::Sleep(Duration::from_millis(5)),
            data(b"b"),
        ]);
        let result = r.read_all().await.unwrap();
        assert_eq!(result, b"ab");
    });

    runtime_test!(read_until_finds_newline, {
        let mut r = buffered(&[data(b"hello\nworld")]);
        let result = r.read_until(b"\n").await.unwrap();
        assert_eq!(result, b"hello\n");
        assert_eq!(r.buffer(), b"world");
    });

    runtime_test!(read_until_multi_byte_delimiter, {
        let mut r = buffered(&[data(b"foo::bar::baz")]);
        let result = r.read_until(b"::").await.unwrap();
        assert_eq!(result, b"foo::");
        assert_eq!(r.buffer(), b"bar::baz");
    });

    runtime_test!(read_until_delimiter_across_chunks, {
        let mut r = buffered(&[data(b"hel"), data(b"lo\nworld")]);
        let result = r.read_until(b"\n").await.unwrap();
        assert_eq!(result, b"hello\n");
    });

    runtime_test!(read_until_eof_before_delimiter, {
        let mut r = buffered(&[data(b"no newline")]);
        let err = r.read_until(b"\n").await.unwrap_err();
        assert!(matches!(err, Error::UnexpectedEof));
    });

    runtime_test!(read_until_consecutive_calls, {
        let mut r = buffered(&[data(b"first\nsecond\n")]);
        let r1 = r.read_until(b"\n").await.unwrap();
        assert_eq!(r1, b"first\n");
        let r2 = r.read_until(b"\n").await.unwrap();
        assert_eq!(r2, b"second\n");
    });

    runtime_test!(read_until_with_sleep, {
        let mut r = buffered(&[
            data(b"waiting"),
            ReadAction::Sleep(Duration::from_millis(10)),
            data(b"\nrest"),
        ]);
        let result = r.read_until(b"\n").await.unwrap();
        assert_eq!(result, b"waiting\n");
        assert_eq!(r.buffer(), b"rest");
    });

    runtime_test!(read_u8_single_byte, {
        let mut r = buffered(&[data(&[0x42])]);
        assert_eq!(r.read_u8().await.unwrap(), 0x42);
    });

    runtime_test!(read_i8_negative, {
        let mut r = buffered(&[data(&[0xFF])]);
        assert_eq!(r.read_i8().await.unwrap(), -1);
    });

    runtime_test!(read_u32_be_value, {
        let mut r = buffered(&[data(&0xDEAD_BEEFu32.to_be_bytes())]);
        assert_eq!(r.read_u32_be().await.unwrap(), 0xDEAD_BEEF);
    });

    runtime_test!(read_u32_little_endian, {
        let mut r = buffered(&[data(&0xDEAD_BEEFu32.to_le_bytes())]);
        assert_eq!(r.read_u32_le().await.unwrap(), 0xDEAD_BEEF);
    });

    runtime_test!(read_i16_le_negative, {
        let mut r = buffered(&[data(&(-1234i16).to_le_bytes())]);
        assert_eq!(r.read_i16_le().await.unwrap(), -1234);
    });

    runtime_test!(read_u64_le, {
        let mut r = buffered(&[data(&0x0102_0304_0506_0708u64.to_le_bytes())]);
        assert_eq!(r.read_u64_le().await.unwrap(), 0x0102_0304_0506_0708);
    });

    runtime_test!(read_f32_be_value, {
        let val: f32 = 3.14;
        let mut r = buffered(&[data(&val.to_be_bytes())]);
        let result = r.read_f32_be().await.unwrap();
        assert!((result - val).abs() < f32::EPSILON);
    });

    runtime_test!(read_f64_little_endian, {
        let val: f64 = 2.718_281_828;
        let mut r = buffered(&[data(&val.to_le_bytes())]);
        let result = r.read_f64_le().await.unwrap();
        assert!((result - val).abs() < f64::EPSILON);
    });

    runtime_test!(read_u32_be_eof_mid_read, {
        let mut r = buffered(&[data(&[0x01])]);
        let err = r.read_u32_be().await.unwrap_err();
        assert!(matches!(err, Error::UnexpectedEof));
    });
}
