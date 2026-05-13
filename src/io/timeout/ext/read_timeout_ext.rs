use tokio::io::AsyncRead;

use crate::io::timeout::PwnTimeout;

use super::read_int_timeout::{ReadF32LeTimeout, ReadF32Timeout, ReadF64LeTimeout, ReadF64Timeout};
use super::read_int_timeout::{
    ReadI128LeTimeout, ReadI128Timeout, ReadI16LeTimeout, ReadI16Timeout, ReadI32LeTimeout,
    ReadI32Timeout, ReadI64LeTimeout, ReadI64Timeout, ReadI8Timeout,
};
use super::read_int_timeout::{
    ReadU128LeTimeout, ReadU128Timeout, ReadU16LeTimeout, ReadU16Timeout, ReadU32LeTimeout,
    ReadU32Timeout, ReadU64LeTimeout, ReadU64Timeout, ReadU8Timeout,
};

use bytes::BufMut;

impl<R: AsyncRead + ?Sized> TimeoutReadExt for R {}

macro_rules! read_impl {
    (
        $(
            $(#[doc = $doc:expr])*
            fn $name:ident(&mut self, timeout: impl Into<PwnTimeout>) -> $($fut:ident)*;
        )*
    ) => {
        $(
            $(#[doc = $doc])*
            fn $name(&mut self, timeout: impl Into<PwnTimeout>) -> $($fut)*<&mut Self> where Self: Unpin {
                $($fut)*::new(self, timeout)
            }
        )*
    }
}

/// Extension trait adding timeout-aware read methods to `AsyncRead` types.
pub trait TimeoutReadExt: AsyncRead {
    /// Read into a byte slice with a timeout.
    fn read_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: impl Into<PwnTimeout>,
    ) -> super::read_timeout::ReadTimeout<'a, Self>
    where
        Self: Unpin,
    {
        super::read_timeout::read_timeout(self, buf, timeout)
    }

    /// Read into a `BufMut` with a timeout.
    fn read_buf_timeout<'a, B>(
        &'a mut self,
        buf: &'a mut B,
        timeout: impl Into<PwnTimeout>,
    ) -> super::read_buf_timeout::ReadBufTimeout<'a, Self, B>
    where
        Self: Unpin,
        B: BufMut + ?Sized,
    {
        super::read_buf_timeout::read_buf_timeout(self, buf, timeout)
    }

    /// Read exactly enough bytes to fill the buffer, with a timeout.
    fn read_exact_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        timeout: impl Into<PwnTimeout>,
    ) -> super::read_exact_timeout::ReadExactTimeout<'a, Self>
    where
        Self: Unpin,
    {
        super::read_exact_timeout::read_exact_timeout(self, buf, timeout)
    }

    read_impl! {
        /// Read a `u8` with a timeout.
        fn read_u8_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU8Timeout;
        /// Read an `i8` with a timeout.
        fn read_i8_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI8Timeout;
        /// Read a big-endian `u16` with a timeout.
        fn read_u16_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU16Timeout;
        /// Read a big-endian `i16` with a timeout.
        fn read_i16_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI16Timeout;
        /// Read a big-endian `u32` with a timeout.
        fn read_u32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU32Timeout;
        /// Read a big-endian `i32` with a timeout.
        fn read_i32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI32Timeout;
        /// Read a big-endian `u64` with a timeout.
        fn read_u64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU64Timeout;
        /// Read a big-endian `i64` with a timeout.
        fn read_i64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI64Timeout;
        /// Read a big-endian `u128` with a timeout.
        fn read_u128_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU128Timeout;
        /// Read a big-endian `i128` with a timeout.
        fn read_i128_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI128Timeout;
        /// Read a big-endian `f32` with a timeout.
        fn read_f32_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF32Timeout;
        /// Read a big-endian `f64` with a timeout.
        fn read_f64_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF64Timeout;
        /// Read a little-endian `u16` with a timeout.
        fn read_u16_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU16LeTimeout;
        /// Read a little-endian `i16` with a timeout.
        fn read_i16_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI16LeTimeout;
        /// Read a little-endian `u32` with a timeout.
        fn read_u32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU32LeTimeout;
        /// Read a little-endian `i32` with a timeout.
        fn read_i32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI32LeTimeout;
        /// Read a little-endian `u64` with a timeout.
        fn read_u64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU64LeTimeout;
        /// Read a little-endian `i64` with a timeout.
        fn read_i64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI64LeTimeout;
        /// Read a little-endian `u128` with a timeout.
        fn read_u128_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadU128LeTimeout;
        /// Read a little-endian `i128` with a timeout.
        fn read_i128_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadI128LeTimeout;
        /// Read a little-endian `f32` with a timeout.
        fn read_f32_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF32LeTimeout;
        /// Read a little-endian `f64` with a timeout.
        fn read_f64_le_timeout(&mut self, timeout: impl Into<PwnTimeout>) -> ReadF64LeTimeout;
    }
}
