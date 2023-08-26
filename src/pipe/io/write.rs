use std::{result};

use super::*;
use bytes::Buf;
use thiserror::*;
use tokio::{io::*, time::error::Elapsed};

#[derive(Error, Debug)]
pub enum PipeWriteError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("timeout error")]
    TimeoutError(#[from] Elapsed),
}

pub type Result<T> = result::Result<T, PipeReadError>;

macro_rules! async_impl_method {
    ($name:ident, $ret_type:ty, ($($arg_name:ident: $arg_type:ty),*), ($($call_arg:ident),*)) => {
        pub async fn $name(&self, $($arg_name: $arg_type),*) -> Result<$ret_type> {
            let result = self.write_stream.lock().await.$name($($call_arg),*).await?;
            Ok(result)
        }
    };
}

//TODO: Copy documentation from original fn
impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    async_impl_method!(write, usize, (buf: &[u8]), (buf));
    async_impl_method!(write_buf, usize, (buf: &mut impl Buf), (buf));
    async_impl_method!(write_all, (), (buf: &[u8]), (buf));
    async_impl_method!(write_all_buf, (), (buf: &mut impl Buf), (buf));
    async_impl_method!(write_u8, (), (byte: u8), (byte));
    async_impl_method!(write_i8, (), (byte: i8), (byte));
    async_impl_method!(write_u16, (), (n: u16), (n));
    async_impl_method!(write_i16, (), (n: i16), (n));
    async_impl_method!(write_u32, (), (n: u32), (n));
    async_impl_method!(write_i32, (), (n: i32), (n));
    async_impl_method!(write_u64, (), (n: u64), (n));
    async_impl_method!(write_i64, (), (n: i64), (n));
    async_impl_method!(write_u128, (), (n: u128), (n));
    async_impl_method!(write_i128, (), (n: i128), (n));
    async_impl_method!(write_f32, (), (n: f32), (n));
    async_impl_method!(write_f64, (), (n: f64), (n));
    async_impl_method!(write_u16_le, (), (n: u16), (n));
    async_impl_method!(write_i16_le, (), (n: i16), (n));
    async_impl_method!(write_u32_le, (), (n: u32), (n));
    async_impl_method!(write_i32_le, (), (n: i32), (n));
    async_impl_method!(write_u64_le, (), (n: u64), (n));
    async_impl_method!(write_i64_le, (), (n: i64), (n));
    async_impl_method!(write_u128_le, (), (n: u128), (n));
    async_impl_method!(write_i128_le, (), (n: i128), (n));
}