use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::io::PipeError;
use crate::io::timeout::HasTimeout;

pub trait PipeReadExt: AsyncWrite + HasTimeout + Unpin
{
    async fn write_line(&mut self, text: &dyn AsRef<[u8]>) -> Result<usize, PipeError>
    {
        let mut res = Vec::with_capacity(text.as_ref().len() + 1);
        res.extend_from_slice(text.as_ref());
        res.push(b'\n');
        let size = self.write(&res).await?;
        Ok(size)
    }

    async fn write_line_crlf(&mut self, text: &dyn AsRef<[u8]>) -> Result<usize, PipeError>
    {
        let mut res = Vec::with_capacity(text.as_ref().len() + 2);
        res.extend_from_slice(text.as_ref());
        res.push(b'\r');
        res.push(b'\n');
        let size = self.write(&res).await?;
        Ok(size)
    }
}