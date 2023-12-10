use crate::io::cache::HasCache;
use crate::io::timeout::HasTimeout;
use crate::io::PipeError;
use ascii::AsciiString;
use tokio::io::{AsyncRead, AsyncReadExt};

const BLOCK_SIZE: usize = 4096;

pub trait PipeReadExt: AsyncRead + HasCache + HasTimeout + Unpin {
    async fn recv(&mut self) -> Result<Vec<u8>, PipeError> {
        let mut data = [0u8; BLOCK_SIZE];
        let size = self.read(&mut data).await?;
        Ok(Vec::from(&data[..size]))
    }

    async fn recvas(&mut self) -> Result<AsciiString, PipeError> {
        let mut data = [0u8; BLOCK_SIZE];
        let size = self.read(&mut data).await?;
        Ok(AsciiString::from_ascii(&data[..size])?)
    }

    async fn recvus(&mut self) -> Result<String, PipeError> {
        let mut data = [0u8; BLOCK_SIZE];
        let size = self.read(&mut data).await?;
        Ok(String::from_utf8(data[..size].to_vec())?)
    }
}
