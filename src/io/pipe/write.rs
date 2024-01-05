use crate::io::timeout::HasTimeout;
use crate::io::PipeError;
use crossterm::Command;
use tokio::io::{AsyncWrite, AsyncWriteExt};

impl<T> PipeWriteExt for T where T: AsyncWrite + HasTimeout + Unpin {}

pub trait PipeWriteExt: AsyncWrite + HasTimeout + Unpin {
    async fn write_line<T: AsRef<[u8]>>(&mut self, text: T) -> Result<usize, PipeError> {
        // to_vec is used so we dont accidentally trigger
        // flush if user did not wrap writer into BufWriter
        let mut res = text.as_ref().to_vec();
        res.push(b'\n');
        let size = self.write(&res).await?;
        self.flush().await?;
        Ok(size)
    }

    async fn write_line_crlf<T: AsRef<[u8]>>(&mut self, text: T) -> Result<usize, PipeError> {
        let mut res = text.as_ref().to_vec();
        res.push(b'\r');
        res.push(b'\n');
        let size = self.write(&res).await?;
        self.flush().await?;
        Ok(size)
    }

    async fn write_flush<T: AsRef<[u8]>>(&mut self, data: T) -> Result<usize, PipeError> {
        let size = self.write(data.as_ref()).await?;
        self.flush().await?;
        Ok(size)
    }

    async fn write_all_flush<T: AsRef<[u8]>>(&mut self, data: T) -> Result<(), PipeError> {
        self.write_all(data.as_ref()).await?;
        self.flush().await?;
        Ok(())
    }


    async fn write_ansi_command<T: Command>(&mut self, command: T) -> Result<usize, PipeError> {
        let mut ansi_command = String::new();
        command.write_ansi(&mut ansi_command)?;
        let size = self.write(ansi_command.as_bytes()).await?;
        self.flush().await?;
        Ok(size)
    }
}
