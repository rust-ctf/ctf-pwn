use std::time::Duration;

use crate::io::{AsyncCacheRead, AsyncReadCacheTimeoutExt, AsyncReadTimeoutExt, PipeError};
use ascii::AsciiString;
use tokio::io::AsyncRead;

pub trait PipeRead: AsyncRead + AsyncCacheRead{
    fn get_timeout(&self) -> Duration;
    fn set_timeout(&mut self, timeout: Duration);
    fn get_block_size(&self) -> usize;
    fn set_block_size(&mut self, block_size: usize);
}

impl<R: PipeRead> PipeReadExt for R {}

pub trait PipeReadExt: PipeRead {
    async fn recv(&mut self) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        let mut data = vec![0u8; self.get_block_size()];
        let _ = self
            .read_exact_timeout(&mut data, self.get_timeout())
            .await?;
        Ok(data)
    }

    async fn recv_until<T: AsRef<[u8]>>(
        &mut self,
        delim: T,
        drop: bool,
    ) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        let mut buf = Vec::new();
        let delim_len = delim.as_ref().len();
        self.read_until_timeout(delim, &mut buf, self.get_timeout())
            .await?;
        if drop {
            buf.drain(buf.len() - delim_len..);
        }
        Ok(buf)
    }

    async fn recv_until_regex(&mut self, pattern: &str, drop: bool) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        let mut buf = Vec::new();
        let (_, match_len) = self
            .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
            .await?;
        if drop {
            buf.drain(buf.len() - match_len..);
        }
        Ok(buf)
    }

    async fn recv_regex(&mut self, pattern: &str) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        let mut buf = Vec::new();
        let (_, match_len) = self
            .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
            .await?;
        buf.drain(..buf.len() - match_len);
        Ok(buf)
    }

    async fn recv_until_regex_split(
        &mut self,
        pattern: &str,
    ) -> Result<(Vec<u8>, Vec<u8>), PipeError>
    where
        Self: Unpin,
    {
        let mut buf = Vec::new();
        let (_, match_len) = self
            .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
            .await?;
        let (data, mch) = buf.split_at(buf.len() - match_len);
        Ok((data.to_vec(), mch.to_vec()))
    }

    async fn recv_line(&mut self) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        self.recv_until(b"\n", true).await
    }

    async fn recv_line_crlf(&mut self) -> Result<Vec<u8>, PipeError>
    where
        Self: Unpin,
    {
        self.recv_until(b"\r\n", true).await
    }

    async fn recv_utf8(&mut self) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv().await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_until_utf8<T: AsRef<[u8]>>(
        &mut self,
        delim: T,
        drop: bool,
    ) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_until(delim, drop).await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_until_regex_utf8(
        &mut self,
        pattern: &str,
        drop: bool,
    ) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_until_regex(pattern, drop).await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_regex_utf8(&mut self, pattern: &str) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_regex(pattern).await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_line_utf8(&mut self) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_line().await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_line_crlf_utf8(&mut self) -> Result<String, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_line_crlf().await?;
        Ok(String::from_utf8(data)?)
    }

    async fn recv_ascii(&mut self) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv().await?;
        Ok(AsciiString::from_ascii(data)?)
    }

    async fn recv_until_ascii<T: AsRef<[u8]>>(
        &mut self,
        delim: T,
        drop: bool,
    ) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_until(delim, drop).await?;
        Ok(AsciiString::from_ascii(data)?)
    }

    async fn recv_until_regex_ascii(
        &mut self,
        pattern: &str,
        drop: bool,
    ) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_until_regex(pattern, drop).await?;
        Ok(AsciiString::from_ascii(data)?)
    }

    async fn recv_regex_ascii(&mut self, pattern: &str) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_regex(pattern).await?;
        Ok(AsciiString::from_ascii(data)?)
    }

    async fn recv_line_ascii(&mut self) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_line().await?;
        Ok(AsciiString::from_ascii(data)?)
    }

    async fn recv_line_crlf_ascii(&mut self) -> Result<AsciiString, PipeError>
    where
        Self: Unpin,
    {
        let data = self.recv_line_crlf().await?;
        Ok(AsciiString::from_ascii(data)?)
    }
}


