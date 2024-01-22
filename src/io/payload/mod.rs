mod payload;
mod write;
mod steps;
mod payloads;
mod builder;
mod arch;

pub(crate) use builder::*;

pub use payload::*;
pub use write::*;
use crate::io::{PipeError, PipeRead, PipeReadExt, PipeWrite};


pub trait PayloadAction
{
    type ReturnType;

    async fn execute<R:PipeRead + Unpin, W:PipeWrite + Unpin>(&self, reader: &mut R, writer: &mut W) -> Result<Self::ReturnType, PipeError>;
}
// pub trait RetPayload<T>
// {
//     async fn execute_ret<R:PipeRead + Unpin, W:PipeWrite + Unpin>(&self, reader: &mut R, writer: &mut W) -> Result<T, PipeError>;
// }
//

// async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(&self, reader: &mut R, writer: &mut W) -> Result<(), PipeError> {
//     let _ = self.execute_ret(reader,writer).await?;
//     Ok(())
// }