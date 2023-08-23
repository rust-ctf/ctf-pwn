use std::result;

use super::*;
use bytes::BufMut;
use thiserror::*;
use tokio::{io::*, time::error::Elapsed};

#[derive(Error, Debug)]
pub enum PipeReadError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("timeout error")]
    TimeoutError(#[from] Elapsed),
}

pub type Result<T> = result::Result<T, PipeReadError>;

macro_rules! async_impl_method {
    ($name:ident, $ret_type:ty, ($($arg_name:ident: $arg_type:ty),*), ($($call_arg:ident),*)) => {
        pub async fn $name(&self, $($arg_name: $arg_type),*) -> Result<$ret_type> {
            let result = self.read_stream.lock().await.$name($($call_arg),*).await?;
            Ok(result)
        }
    };
}
impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    async_impl_method!(read, usize, (buf: &mut [u8]), (buf));
    async_impl_method!(read_buf, usize, (buf: &mut impl BufMut), (buf));
    async_impl_method!(read_exact, usize, (buf: &mut [u8]), (buf));
    async_impl_method!(read_to_end, usize, (buf: &mut Vec<u8>), (buf));
    async_impl_method!(read_to_string, usize, (buf: &mut String), (buf));
    async_impl_method!(read_u8, u8, (), ());
    async_impl_method!(read_u16, u16, (), ());
    async_impl_method!(read_u32, u32, (), ());
    async_impl_method!(read_u64, u64, (), ());
    async_impl_method!(read_u128, u128, (), ());
    async_impl_method!(read_i8, i8, (), ());
    async_impl_method!(read_i16, i16, (), ());
    async_impl_method!(read_i32, i32, (), ());
    async_impl_method!(read_i64, i64, (), ());
    async_impl_method!(read_i128, i128, (), ());
    async_impl_method!(read_u16_le, u16, (), ());
    async_impl_method!(read_u32_le, u32, (), ());
    async_impl_method!(read_u64_le, u64, (), ());
    async_impl_method!(read_u128_le, u128, (), ());
    async_impl_method!(read_i16_le, i16, (), ());
    async_impl_method!(read_i32_le, i32, (), ());
    async_impl_method!(read_i64_le, i64, (), ());
    async_impl_method!(read_i128_le, i128, (), ());
    async_impl_method!(read_f32, f32, (), ());
    async_impl_method!(read_f64, f64, (), ());
}