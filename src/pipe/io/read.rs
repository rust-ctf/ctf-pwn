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

enum ReadUntilActuion {
    Restore,
    Return,
    Discard,
}

//TODO: Copy documentation from original fn
impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    pub async fn read_all(&self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 1024];
        let mut result = Vec::with_capacity(1024);
        loop {
            match self.read(&mut buf).await {
                Err(PipeReadError::TimeoutError(_)) => break,
                Err(PipeReadError::IoError(e)) if e.kind() == io::ErrorKind::TimedOut => break,
                Err(e) => return Err(e),
                Ok(0) => break, //EOF
                Ok(len) => result.extend_from_slice(&buf[..len]),
            }
        }
        Ok(result)
    }

    pub async fn read_until(&self, end: &[u8]) -> Result<Vec<u8>> {
        self.read_until_internal(end, ReadUntilActuion::Return)
            .await
    }

    pub async fn read_until_restore(&self, end: &[u8]) -> Result<Vec<u8>> {
        self.read_until_internal(end, ReadUntilActuion::Restore)
            .await
    }

    pub async fn read_until_discard(&self, end: &[u8]) -> Result<Vec<u8>> {
        self.read_until_internal(end, ReadUntilActuion::Discard)
            .await
    }

    async fn read_until_internal(&self, end: &[u8], action: ReadUntilActuion) -> Result<Vec<u8>> {
        if end.len() == 0 {
            return Ok(Vec::new());
        }
        let mut result = Vec::with_capacity(1024);
        let mut position = 0usize;
        let mut buf = [0u8; 1024];
        loop {
            match self.read(&mut buf).await {
                Err(e) => {
                    self.read_stream.lock().await.restore_slice(&result);
                    return Err(e);
                }
                Ok(0) => {
                    self.read_stream.lock().await.restore_slice(&result);
                    return Err(PipeReadError::IoError(Error::new(
                        ErrorKind::UnexpectedEof,
                        "Reached end of stream without match",
                    )));
                }
                Ok(len) => result.extend_from_slice(&buf[..len]),
            }

            //TODO: Optimize
            for w in result[position..].windows(end.len()) {
                if w == end {
                    let (slice_start, truncate_start) = match action {
                        ReadUntilActuion::Return => (position + end.len(), position + end.len()),
                        ReadUntilActuion::Restore => (position, position),
                        ReadUntilActuion::Discard => (position + end.len(), position),
                    };
                    self.read_stream
                        .lock()
                        .await
                        .restore_slice(&result[(slice_start + end.len())..]);
                    result.truncate(truncate_start);
                    return Ok(result);
                }
                position += 1;
            }
        }
    }

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
