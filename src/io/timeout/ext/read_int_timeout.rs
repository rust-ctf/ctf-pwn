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
