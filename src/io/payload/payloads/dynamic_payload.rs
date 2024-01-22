use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};

pub struct DynamicPayload<P, E, R>
{
    prev_payload: P,
    action: fn(E) -> R
}

impl<P, E, R> DynamicPayload<P, E, R>
    where P: PayloadAction<ReturnType=E>, R: PayloadAction
{
    pub fn new(prev_payload: P, action: fn(E) -> R) -> DynamicPayload<P, E, R>
    {
        DynamicPayload { prev_payload, action }
    }
}


impl<P, E, T> PayloadAction for DynamicPayload<P,E,T>
where P: PayloadAction<ReturnType=E>, T: PayloadAction
{
    type ReturnType = T::ReturnType;

    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(&self, reader: &mut R, writer: &mut W) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(reader, writer).await?;
        let new_payload = (self.action)(result);
        new_payload.execute(reader, writer).await
    }
}