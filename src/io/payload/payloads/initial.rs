use crate::io::*;

#[derive(Clone)]
pub struct Initial {}

impl Sendable for Initial {}
impl Readable for Initial {}

impl Initial {
    pub fn new() -> Initial {
        Initial {}
    }
}

impl PayloadAction for Initial {
    type ReturnType = ();

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(
        &self,
        _pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        Ok(())
    }
}
