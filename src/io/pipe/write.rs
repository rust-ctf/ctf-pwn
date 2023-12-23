use crate::io::timeout::HasTimeout;
use crate::io::PipeError;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub trait PipeWriteExt: AsyncWrite + HasTimeout + Unpin {
    async fn write_line(&mut self, text: &dyn AsRef<[u8]>) -> Result<usize, PipeError> {
        // to_vec is used so we dont accidentally trigger
        // flush if user did not wrap writer into BufWriter
        let mut res = text.as_ref().to_vec();
        res.push(b'\n');
        let size = self.write(&res).await?;
        Ok(size)
    }

    async fn write_line_crlf(&mut self, text: &dyn AsRef<[u8]>) -> Result<usize, PipeError> {
        let mut res = text.as_ref().to_vec();
        res.push(b'\r');
        res.push(b'\n');
        let size = self.write(&res).await?;
        Ok(size)
    }
}
