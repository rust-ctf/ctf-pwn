mod arch;
mod builder;
mod payload;
mod payloads;
mod steps;
mod write;

pub(crate) use builder::*;

use crate::io::{AsyncCacheRead, PipeError, PipeRead, PipeReadExt, PipeWrite, PipeWriteExt};
pub use payload::*;
pub use write::*;

pub trait PayloadAction {
    type ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError>;
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
