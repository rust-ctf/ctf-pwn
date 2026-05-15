use crate::{io::timeout::*, timeout_ready};
use bytes::Buf;
use pin_project_lite::pin_project;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

macro_rules! reader {
    ($name:ident, $ty:ty, $reader:ident) => {
        reader!($name, $ty, $reader, size_of::<$ty>());
    };
    ($name:ident, $ty:ty, $reader:ident, $bytes:expr) => {
        pin_project! {
            pub struct $name<R> {
                #[pin]
                src: R,
                buf: [u8; $bytes],
                read: usize,
                #[pin]
                delay: Option<BoxSleep>,
                callback: PwnTimeout,
                #[pin]
                _pin: PhantomPinned,
            }
        }

        impl<R> $name<R> {
            pub(crate) fn new(src: R, timeout: impl Into<PwnTimeout>) -> Self {
                $name {
                    src,
                    buf: [0; $bytes],
                    read: 0,
                    delay: None,
                    callback: timeout.into(),
                    _pin: PhantomPinned,
                }
            }
        }

        impl<R> Future for $name<R>
        where
            R: AsyncRead,
        {
            type Output = Result<$ty, IOTimeoutError>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();

                if *me.read == $bytes {
                    return Poll::Ready(Ok(Buf::$reader(&mut &me.buf[..])));
                }

                while *me.read < $bytes {
                    let mut buf = ReadBuf::new(&mut me.buf[*me.read..]);

                    timeout_ready!(
                        me.src.as_mut().poll_read(cx, &mut buf),
                        me.delay,
                        me.callback,
                        cx
                    )?;

                    let n = buf.filled().len();
                    if n == 0 {
                        return Poll::Ready(Err(IOTimeoutError::UnexpectedEof));
                    }
                    *me.read += n;
                }

                let num = Buf::$reader(&mut &me.buf[..]);

                Poll::Ready(Ok(num))
            }
        }
    };
}

macro_rules! reader8 {
    ($name:ident, $ty:ty) => {
        pin_project! {
            pub struct $name<R> {
                #[pin]
                reader: R,
                #[pin]
                delay: Option<BoxSleep>,
                callback: PwnTimeout,
                #[pin]
                _pin: PhantomPinned,
            }
        }

        impl<R> $name<R> {
            pub(crate) fn new(reader: R, timeout: impl Into<PwnTimeout>) -> $name<R> {
                $name {
                    reader,
                    delay: None,
                    callback: timeout.into(),
                    _pin: PhantomPinned,
                }
            }
        }

        impl<R> Future for $name<R>
        where
            R: AsyncRead,
        {
            type Output = Result<$ty, IOTimeoutError>;

            #[expect(clippy::allow_attributes, reason = "expect cannot be used here as not all lints fire in each macro expansion")]
            #[allow(
                trivial_numeric_casts,
                clippy::cast_possible_wrap,
                reason = "macro is used for both u8 and i8"
            )]
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();

                let mut buf = [0; 1];
                let mut buf = ReadBuf::new(&mut buf);

                timeout_ready!(
                    me.reader.as_mut().poll_read(cx, &mut buf),
                    me.delay,
                    me.callback,
                    cx
                )?;

                if buf.filled().is_empty() {
                    return Poll::Ready(Err(IOTimeoutError::UnexpectedEof));
                }

                Poll::Ready(Ok(buf.filled()[0] as $ty))
            }
        }
    };
}

reader8!(ReadU8Timeout, u8);
reader8!(ReadI8Timeout, i8);

reader!(ReadU16Timeout, u16, get_u16);
reader!(ReadU32Timeout, u32, get_u32);
reader!(ReadU64Timeout, u64, get_u64);
reader!(ReadU128Timeout, u128, get_u128);

reader!(ReadI16Timeout, i16, get_i16);
reader!(ReadI32Timeout, i32, get_i32);
reader!(ReadI64Timeout, i64, get_i64);
reader!(ReadI128Timeout, i128, get_i128);

reader!(ReadF32Timeout, f32, get_f32);
reader!(ReadF64Timeout, f64, get_f64);

reader!(ReadU16LeTimeout, u16, get_u16_le);
reader!(ReadU32LeTimeout, u32, get_u32_le);
reader!(ReadU64LeTimeout, u64, get_u64_le);
reader!(ReadU128LeTimeout, u128, get_u128_le);

reader!(ReadI16LeTimeout, i16, get_i16_le);
reader!(ReadI32LeTimeout, i32, get_i32_le);
reader!(ReadI64LeTimeout, i64, get_i64_le);
reader!(ReadI128LeTimeout, i128, get_i128_le);

reader!(ReadF32LeTimeout, f32, get_f32_le);
reader!(ReadF64LeTimeout, f64, get_f64_le);

#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::io::{
        test::{AsyncTestReader, TestAction},
        timeout::{IOTimeoutError, TimeoutReadExt},
    };

    // big-endian
    #[tokio::test]
    async fn read_u8_timeout_reads_value() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0x42])]);
        let val = reader.read_u8_timeout(Duration::from_secs(1)).await.expect("read_u8 should succeed");
        assert_eq!(val, 0x42);
    }

    #[tokio::test]
    async fn read_i8_timeout_reads_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0xFF])]);
        let val = reader.read_i8_timeout(Duration::from_secs(1)).await.expect("read_i8 should succeed");
        assert_eq!(val, -1);
    }

    #[tokio::test]
    async fn read_u16_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0x01, 0x02])]);
        let val = reader.read_u16_timeout(Duration::from_secs(1)).await.expect("read_u16 should succeed");
        assert_eq!(val, 0x0102);
    }

    #[tokio::test]
    async fn read_i16_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0xFF, 0xFE])]);
        let val = reader.read_i16_timeout(Duration::from_secs(1)).await.expect("read_i16 should succeed");
        assert_eq!(val, -2);
    }

    #[tokio::test]
    async fn read_u32_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0x00, 0x00, 0x01, 0x00])]);
        let val = reader.read_u32_timeout(Duration::from_secs(1)).await.expect("read_u32 should succeed");
        assert_eq!(val, 256);
    }

    #[tokio::test]
    async fn read_i32_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0xFF, 0xFF, 0xFF, 0xFF])]);
        let val = reader.read_i32_timeout(Duration::from_secs(1)).await.expect("read_i32 should succeed");
        assert_eq!(val, -1);
    }

    #[tokio::test]
    async fn read_u64_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(1234u64.to_be_bytes().to_vec())]);
        let val = reader.read_u64_timeout(Duration::from_secs(1)).await.expect("read_u64 should succeed");
        assert_eq!(val, 1234);
    }

    #[tokio::test]
    async fn read_i64_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-999i64).to_be_bytes().to_vec())]);
        let val = reader.read_i64_timeout(Duration::from_secs(1)).await.expect("read_i64 should succeed");
        assert_eq!(val, -999);
    }

    #[tokio::test]
    async fn read_u128_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(42u128.to_be_bytes().to_vec())]);
        let val = reader.read_u128_timeout(Duration::from_secs(1)).await.expect("read_u128 should succeed");
        assert_eq!(val, 42);
    }

    #[tokio::test]
    async fn read_i128_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-1i128).to_be_bytes().to_vec())]);
        let val = reader.read_i128_timeout(Duration::from_secs(1)).await.expect("read_i128 should succeed");
        assert_eq!(val, -1);
    }

    // little-endian
    #[tokio::test]
    async fn read_u16_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0x02, 0x01])]);
        let val = reader.read_u16_le_timeout(Duration::from_secs(1)).await.expect("read_u16_le should succeed");
        assert_eq!(val, 0x0102);
    }

    #[tokio::test]
    async fn read_i16_le_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-2i16).to_le_bytes().to_vec())]);
        let val = reader.read_i16_le_timeout(Duration::from_secs(1)).await.expect("read_i16_le should succeed");
        assert_eq!(val, -2);
    }

    #[tokio::test]
    async fn read_u32_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(256u32.to_le_bytes().to_vec())]);
        let val = reader.read_u32_le_timeout(Duration::from_secs(1)).await.expect("read_u32_le should succeed");
        assert_eq!(val, 256);
    }

    #[tokio::test]
    async fn read_i32_le_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-1i32).to_le_bytes().to_vec())]);
        let val = reader.read_i32_le_timeout(Duration::from_secs(1)).await.expect("read_i32_le should succeed");
        assert_eq!(val, -1);
    }

    #[tokio::test]
    async fn read_u64_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(1234u64.to_le_bytes().to_vec())]);
        let val = reader.read_u64_le_timeout(Duration::from_secs(1)).await.expect("read_u64_le should succeed");
        assert_eq!(val, 1234);
    }

    #[tokio::test]
    async fn read_i64_le_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-999i64).to_le_bytes().to_vec())]);
        let val = reader.read_i64_le_timeout(Duration::from_secs(1)).await.expect("read_i64_le should succeed");
        assert_eq!(val, -999);
    }

    #[tokio::test]
    async fn read_u128_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(42u128.to_le_bytes().to_vec())]);
        let val = reader.read_u128_le_timeout(Duration::from_secs(1)).await.expect("read_u128_le should succeed");
        assert_eq!(val, 42);
    }

    #[tokio::test]
    async fn read_i128_le_timeout_negative() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data((-1i128).to_le_bytes().to_vec())]);
        let val = reader.read_i128_le_timeout(Duration::from_secs(1)).await.expect("read_i128_le should succeed");
        assert_eq!(val, -1);
    }

    // floats
    #[tokio::test]
    async fn read_f32_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(3.14f32.to_be_bytes().to_vec())]);
        let val = reader.read_f32_timeout(Duration::from_secs(1)).await.expect("read_f32 should succeed");
        assert!((val - 3.14).abs() < 0.001);
    }

    #[tokio::test]
    async fn read_f64_timeout_big_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(2.718281828f64.to_be_bytes().to_vec())]);
        let val = reader.read_f64_timeout(Duration::from_secs(1)).await.expect("read_f64 should succeed");
        assert!((val - 2.718281828).abs() < 1e-9);
    }

    #[tokio::test]
    async fn read_f32_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(3.14f32.to_le_bytes().to_vec())]);
        let val = reader.read_f32_le_timeout(Duration::from_secs(1)).await.expect("read_f32_le should succeed");
        assert!((val - 3.14).abs() < 0.001);
    }

    #[tokio::test]
    async fn read_f64_le_timeout_little_endian() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(2.718281828f64.to_le_bytes().to_vec())]);
        let val = reader.read_f64_le_timeout(Duration::from_secs(1)).await.expect("read_f64_le should succeed");
        assert!((val - 2.718281828).abs() < 1e-9);
    }

    // cross-chunk reads
    #[tokio::test]
    async fn read_u32_timeout_across_chunks() {
        let bytes = 0xDEADBEEFu32.to_be_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(bytes[..2].to_vec()),
            TestAction::Data(bytes[2..].to_vec()),
        ]);
        let val = reader.read_u32_timeout(Duration::from_secs(1)).await.expect("read_u32 should succeed");
        assert_eq!(val, 0xDEADBEEF);
    }

    #[tokio::test]
    async fn read_u64_le_timeout_across_chunks() {
        let bytes = 0x0102030405060708u64.to_le_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(bytes[..3].to_vec()),
            TestAction::Data(bytes[3..].to_vec()),
        ]);
        let val = reader.read_u64_le_timeout(Duration::from_secs(1)).await.expect("read_u64_le should succeed");
        assert_eq!(val, 0x0102030405060708);
    }

    // EOF and timeout errors
    #[tokio::test]
    async fn read_u8_timeout_eof() {
        let mut reader = AsyncTestReader::new(&[]);
        let err = reader.read_u8_timeout(Duration::from_secs(1)).await.expect_err("should error on EOF");
        assert!(matches!(err, IOTimeoutError::UnexpectedEof));
    }

    #[tokio::test]
    async fn read_u32_timeout_short_eof() {
        let mut reader = AsyncTestReader::new(&[TestAction::Data(vec![0x01, 0x02])]);
        let err = reader.read_u32_timeout(Duration::from_secs(1)).await.expect_err("should error on short read");
        assert!(matches!(err, IOTimeoutError::UnexpectedEof));
    }

    #[tokio::test]
    async fn read_u32_timeout_times_out() {
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(vec![0x01]),
            TestAction::Sleep(Duration::from_secs(10)),
        ]);
        let err = reader.read_u32_timeout(Duration::from_millis(50)).await.expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }

    #[tokio::test]
    async fn read_u32_delayed_bytes_arrive_in_time() {
        let bytes = 0xCAFEBABEu32.to_be_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(bytes[..2].to_vec()),
            TestAction::Sleep(Duration::from_millis(30)),
            TestAction::Data(bytes[2..].to_vec()),
        ]);
        let val = reader.read_u32_timeout(Duration::from_millis(500)).await.expect("should arrive in time");
        assert_eq!(val, 0xCAFEBABE);
    }

    #[tokio::test]
    async fn read_u32_delayed_bytes_arrive_too_late() {
        let bytes = 0xCAFEBABEu32.to_be_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(bytes[..2].to_vec()),
            TestAction::Sleep(Duration::from_millis(200)),
            TestAction::Data(bytes[2..].to_vec()),
        ]);
        let err = reader.read_u32_timeout(Duration::from_millis(50)).await.expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }

    #[tokio::test]
    async fn read_u64_le_byte_by_byte_with_delays_succeeds() {
        let bytes = 0x0807060504030201u64.to_le_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(vec![bytes[0]]),
            TestAction::Sleep(Duration::from_millis(10)),
            TestAction::Data(vec![bytes[1]]),
            TestAction::Sleep(Duration::from_millis(10)),
            TestAction::Data(bytes[2..].to_vec()),
        ]);
        let val = reader.read_u64_le_timeout(Duration::from_millis(500)).await.expect("should succeed");
        assert_eq!(val, 0x0807060504030201);
    }

    #[tokio::test]
    async fn read_u64_le_byte_by_byte_delays_exceed_timeout() {
        let bytes = 0x0807060504030201u64.to_le_bytes();
        let mut reader = AsyncTestReader::new(&[
            TestAction::Data(vec![bytes[0]]),
            TestAction::Sleep(Duration::from_millis(30)),
            TestAction::Data(vec![bytes[1]]),
            TestAction::Sleep(Duration::from_millis(30)),
            TestAction::Data(bytes[2..].to_vec()),
        ]);
        let err = reader.read_u64_le_timeout(Duration::from_millis(50)).await.expect_err("should timeout");
        assert!(matches!(err, IOTimeoutError::Timeout));
    }
}
