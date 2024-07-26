mod read_buf_timeout;
mod read_exact_timeout;
mod read_int_timeout;
mod read_timeout;

use tokio::io::AsyncRead;

use read_int_timeout::{ReadF32LeTimeout, ReadF32Timeout, ReadF64LeTimeout, ReadF64Timeout};
use read_int_timeout::{
    ReadI128LeTimeout, ReadI128Timeout, ReadI16LeTimeout, ReadI16Timeout, ReadI32LeTimeout,
    ReadI32Timeout, ReadI64LeTimeout, ReadI64Timeout, ReadI8Timeout,
};
use read_int_timeout::{
    ReadU128LeTimeout, ReadU128Timeout, ReadU16LeTimeout, ReadU16Timeout, ReadU32LeTimeout,
    ReadU32Timeout, ReadU64LeTimeout, ReadU64Timeout, ReadU8Timeout,
};

use super::PwnTimeout;
use bytes::BufMut;

impl<R: AsyncRead + ?Sized> TimeoutReadExt for R {}

#[macro_export]
macro_rules! timeout_ready {
    ($e:expr $(,)?, $d:expr $(,)?, $c:expr $(,)?, $cx:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => {
                let delay = $d.get_or_insert_with(|| Box::pin($c.timeout()));

                return match delay.as_mut().poll($cx) {
                    Poll::Pending => std::task::Poll::Pending,
                    Poll::Ready(()) => std::task::Poll::Ready(Err(IOTimeoutError::Timeout)),
                };
            }
        }
    };
}

macro_rules! read_impl {
    (
        $(
            fn $name:ident(&mut self, timeout: impl Into<PwnTimeout>) -> $($fut:ident)*;
        )*
    ) => {
        $(
            fn $name(&mut self, timeout: impl Into<PwnTimeout>) -> $($fut)*<&mut Self> where Self: Unpin {
                $($fut)*::new(self, timeout)
            }
        )*
    }
}

pub trait TimeoutReadExt: AsyncRead {
    fn read_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: impl Into<PwnTimeout>,
    ) -> read_timeout::ReadTimeout<'a, Self>
    where
        Self: Unpin,
    {
        read_timeout::read_timeout(self, buf, timeout)
    }

    fn read_buf_timeout<'a, B>(
        &'a mut self,
        buf: &'a mut B,
        timeout: impl Into<PwnTimeout>,
    ) -> read_buf_timeout::ReadBufTimeout<'a, Self, B>
    where
        Self: Unpin,
        B: BufMut + ?Sized,
    {
        read_buf_timeout::read_buf_timeout(self, buf, timeout)
    }

    fn read_exact_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: impl Into<PwnTimeout>,
    ) -> read_exact_timeout::ReadExactTimeout<'a, Self>
    where
        Self: Unpin,
    {
        read_exact_timeout::read_exact_timeout(self, buf, timeout)
    }

    read_impl! {
        fn read_u8_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU8Timeout;
        fn read_i8_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI8Timeout;
        fn read_u16_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU16Timeout;
        fn read_i16_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI16Timeout;
        fn read_u32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU32Timeout;
        fn read_i32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI32Timeout;
        fn read_u64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU64Timeout;
        fn read_i64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI64Timeout;
        fn read_u128_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU128Timeout;
        fn read_i128_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI128Timeout;
        fn read_f32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF32Timeout;
        fn read_f64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF64Timeout;
        fn read_u16_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU16LeTimeout;
        fn read_i16_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI16LeTimeout;
        fn read_u32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU32LeTimeout;
        fn read_i32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI32LeTimeout;
        fn read_u64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU64LeTimeout;
        fn read_i64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI64LeTimeout;
        fn read_u128_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU128LeTimeout;
        fn read_i128_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI128LeTimeout;
        fn read_f32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF32LeTimeout;
        fn read_f64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF64LeTimeout;
    }
}
