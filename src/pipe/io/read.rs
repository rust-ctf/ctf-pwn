use std::result;

use super::*;
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncBufReadExt}, time::error::Elapsed};
use thiserror::*;
use tokio::time::timeout;

#[derive(Error, Debug)]
pub enum PipeReadError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("timeout error")]
    TimeoutError(#[from] Elapsed),
}

pub type Result<T> = result::Result<T, PipeReadError>;

macro_rules! async_read_buf_impl {
    ($($fn_name:ident, $buf_type:ty),+) => {
        $(
            pub async fn $fn_name<'a>(&'a self, buf: &'a mut $buf_type) -> Result<usize> {
                let mut stream = self.read_stream.lock().await;
                let size = match self.read_timeout {
                    Some(read_timeout) => {
                        timeout(read_timeout, stream.$fn_name(buf)).await??
                    },
                    None => {
                        stream.$fn_name(buf).await?
                    },
                };
                Ok(size)
            }
        )+
    };
}

macro_rules! async_read_ext_impl {
    ($($fn_name:ident, $ret_type:ty),+) => {
        $(
            pub async fn $fn_name<'a>(&'a self) -> Result<$ret_type> {
                let mut stream = self.read_stream.lock().await;
                let size = match self.read_timeout {
                    Some(read_timeout) => {
                        timeout(read_timeout, stream.$fn_name()).await??
                    },
                    None => {
                        stream.$fn_name().await?
                    },
                };
                Ok(size)
            }
        )+
    };
}


impl<R: AsyncRead, W: AsyncWrite> Pipe<R,W>
{
   
}
        //Nothing to match
        // if end.len() == 0
        // {
        //     return Ok(vec![]);
        // }
        // let mut buffer = [0u8, 512];
        // let mut result = Vec::new();
        // let mut position = 0usize;
        // loop {
        //     match self.read_buf(&mut buffer) {
        //         Err(_) => {
        //             self.read_cache.extend(result.iter());
        //             return None;
        //         }
        //         Ok((_, 0)) => {
        //             if !wait {
        //                 return None;
        //             }
        //         }
        //         Ok((buffer, len)) => result.extend_from_slice(&buffer[..len]),
        //     }

        //     for w in result[position..].windows(end.len()) {
        //         if w == end {
        //             self.read_cache
        //                 .extend(result[(position + end.len())..].iter());
        //             result.truncate(position);
        //             return Some(result);
        //         }
        //         position += 1;
        //     }
        // }
    //}

    // async_read_buf_impl!(read, [u8]);
    // async_read_buf_impl!(read_exact, [u8]);
    // //async_read_buf_impl!(read_line, String);
    // async_read_ext_impl!(read_i8, i8);
    // async_read_ext_impl!(read_i16_le, i16);
    // async_read_ext_impl!(read_i16, i16);
    // async_read_ext_impl!(read_i32_le, i32);
    // async_read_ext_impl!(read_i32, i32);
    // async_read_ext_impl!(read_i64_le, i64);
    // async_read_ext_impl!(read_i64, i64);
    // async_read_ext_impl!(read_i128_le, i128);
    // async_read_ext_impl!(read_i128, i128);
    // async_read_ext_impl!(read_u8, u8);
    // async_read_ext_impl!(read_u16_le, u16);
    // async_read_ext_impl!(read_u16, u16);
    // async_read_ext_impl!(read_u32_le, u32);
    // async_read_ext_impl!(read_u32, u32);
    // async_read_ext_impl!(read_u64_le, u64);
    // async_read_ext_impl!(read_u64, u64);
    // async_read_ext_impl!(read_u128_le, u128);
    // async_read_ext_impl!(read_u128, u128);
    // async_read_ext_impl!(read_f32_le, f32);
    // async_read_ext_impl!(read_f32, f32);
    // async_read_ext_impl!(read_f64_le, f64);
    // async_read_ext_impl!(read_f64, f64);
//}