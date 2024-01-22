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

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError> {
        Ok(())
    }
}