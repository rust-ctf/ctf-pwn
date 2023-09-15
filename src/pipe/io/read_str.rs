use std::result;
use ascii::AsciiString;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::*;

pub type Result<T> = result::Result<T, PipeReadError>;

const NEWLINE_DELIMITER: &'static [u8;1] = b"\n";
//const NEWLINE_DELIMITER_WINDOWS: &'static [u8;2] = b"\r\n";
impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    pub async fn read_all_utf8(&self) -> Result<String>
    {
        let data = self.read_all().await?;
        let result = String::from_utf8(data)?;
        Ok(result)
    }

    pub async fn read_line_utf8(&self) -> Result<String>
    {
        let data = self.read_until_discard(NEWLINE_DELIMITER.as_ref()).await?;
        let result = String::from_utf8(data)?;
        Ok(result)
    }

    pub async fn read_all_ascii(&self) -> Result<AsciiString>
    {
        let data = self.read_all().await?;
        let result = AsciiString::from_ascii(data)?;
        Ok(result)
    }

    pub async fn read_line_ascii(&self) -> Result<AsciiString>
    {
        let data = self.read_until_discard(NEWLINE_DELIMITER.as_ref()).await?;
        let result = AsciiString::from_ascii(data)?;
        Ok(result)
    }
}