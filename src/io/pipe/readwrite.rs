use crate::io::{Payload, PayloadStep, PipeError, PipeReadExt, PipeWriteExt};
use tokio::io::AsyncWriteExt;

impl<T> PipeReadWriteExt for T where T: PipeReadExt + PipeWriteExt {}

pub trait PipeReadWriteExt: PipeReadExt + PipeWriteExt {
    async fn payload(&mut self, payload: &Payload) -> Result<usize, PipeError>
        where Self: Unpin
    {
        let mut size = 0usize;
        for step in payload.steps() {
            match step {
                PayloadStep::Data(data) => {
                    self.write_all(&data).await?;
                    size += data.len();
                    tokio::io::stdout().write_all(&data).await?;
                }
                PayloadStep::Send() => {
                    self.flush().await?;
                }
                PayloadStep::ReadUntil(data) => {
                    let data = self.recv_until(data, false).await?;
                    for chunk in data.chunks(1024) {
                        tokio::io::stdout().write_all(chunk).await?;
                    }
                }
            }
        }

        self.flush().await?;
        Ok(size)
    }
}
