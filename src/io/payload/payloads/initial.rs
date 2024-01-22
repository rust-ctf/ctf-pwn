use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};

pub struct Initial
{

}

impl Initial
{
    pub fn new() -> Initial
    {
        Initial{}
    }
}

impl PayloadAction for Initial
{
    type ReturnType = ();

    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(&self, reader: &mut R, writer: &mut W) -> Result<Self::ReturnType, PipeError> {
        Ok(())
    }
}