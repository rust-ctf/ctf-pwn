use std::io::ErrorKind;
use std::isize;
use std::ops::DerefMut;
use crate::io::cache::HasCache;
use crate::io::timeout::HasTimeout;
use crate::io::{AsyncCacheRead, AsyncCacheReadExt, AsyncReadCacheTimeoutExt, AsyncReadTimeoutExt, PipeError};
use ascii::AsciiString;
use tokio::io::{AsyncRead, AsyncReadExt};


const BLOCK_SIZE: usize = 4096;

impl<T> PipeReadExt for T where T: AsyncRead + HasCache + HasTimeout + Unpin + AsyncCacheRead {}

pub trait PipeReadExt: AsyncRead + HasCache + HasTimeout + Unpin + AsyncCacheRead{
    async fn recv(&mut self) -> Result<Vec<u8>, PipeError> {
        let mut data = [0u8; BLOCK_SIZE];
        let size = self.read_exact_timeout(&mut data, self.read_timeout().unwrap()).await?;
        Ok(Vec::from(&data[..size]))
    }

    // async fn recv_timeout(&mut self) -> Result<Vec<u8>, PipeError> {
    //     let mut res = Vec::new();
    //     loop {
    //         let mut data = [0u8; BLOCK_SIZE];
    //         match self.deref_mut().read_exact_timeout(&mut data).await {
    //             Ok(0) => break,
    //             Ok(size) => res.extend_from_slice(&data[..size]),
    //             Err(e) if e.kind() == ErrorKind::TimedOut => break,
    //             Err(e) => return Err(e.into()),
    //         }
    //     }
    //
    //     Ok(res)
    // }

    async fn recv_until<T:AsRef<[u8]>>(
        &mut self,
        end: T,
        drop: bool,
    ) -> Result<Vec<u8>, PipeError> {
        let mut buf = Vec::new();
        self.read_until_timeout(end, &mut buf, self.read_timeout().unwrap()).await?;
        Ok(buf)
    }
    //
    // async fn recv_line(&mut self) -> Result<Vec<u8>, PipeError> {
    //     self.recv_until(b"\n", true).await
    // }
    //
    // async fn recv_line_crlf(&mut self) -> Result<Vec<u8>, PipeError> {
    //     self.recv_until(b"\r\n", true).await
    // }
    //
    // async fn recvus(&mut self) -> Result<String, PipeError> {
    //     let data = self.recv().await?;
    //     Ok(String::from_utf8(data)?)
    // }
    //
    // async fn recvus_timeout(&mut self) -> Result<String, PipeError> {
    //     let data = self.recv_timeout().await?;
    //     Ok(String::from_utf8(data)?)
    // }
    // async fn recvus_until(
    //     &mut self,
    //     end: &dyn AsRef<[u8]>,
    //     drop: bool,
    // ) -> Result<String, PipeError> {
    //     let data = self.recv_until(end, drop).await?;
    //     Ok(String::from_utf8(data)?)
    // }
    //
    // async fn recvus_line(&mut self) -> Result<String, PipeError> {
    //     let data = self.recv_line().await?;
    //     Ok(String::from_utf8(data)?)
    // }
    //
    // async fn recvus_line_crlf(&mut self) -> Result<String, PipeError> {
    //     let data = self.recv_line_crlf().await?;
    //     Ok(String::from_utf8(data)?)
    // }
    //
    // async fn recvas(&mut self) -> Result<AsciiString, PipeError> {
    //     let data = self.recv().await?;
    //     Ok(AsciiString::from_ascii(data)?)
    // }
    //
    // async fn recvas_timeout(&mut self) -> Result<AsciiString, PipeError> {
    //     let data = self.recv_timeout().await?;
    //     Ok(AsciiString::from_ascii(data)?)
    // }
    //
    // async fn recvas_until(
    //     &mut self,
    //     end: &dyn AsRef<[u8]>,
    //     drop: bool,
    // ) -> Result<AsciiString, PipeError> {
    //     let data = self.recv_until(end, drop).await?;
    //     Ok(AsciiString::from_ascii(data)?)
    // }
    //
    // async fn recvas_line(&mut self) -> Result<AsciiString, PipeError> {
    //     let data = self.recv_line().await?;
    //     Ok(AsciiString::from_ascii(data)?)
    // }
    //
    // async fn recvas_line_crlf(&mut self) -> Result<AsciiString, PipeError> {
    //     let data = self.recv_line_crlf().await?;
    //     Ok(AsciiString::from_ascii(data)?)
    // }
}
