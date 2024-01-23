use crate::io::*;

pub struct DynamicPayload<P, E, R> {
    prev_payload: P,
    action: fn(E) -> R,
}

impl<P, E, R: Buildable> Buildable for DynamicPayload<P, E, R> where
    DynamicPayload<P, E, R>: PayloadAction
{
}

impl<P, E, R: Sendable> Sendable for DynamicPayload<P, E, R> where
    DynamicPayload<P, E, R>: PayloadAction
{
}

impl<P, E, R: Readable> Readable for DynamicPayload<P, E, R> where
    DynamicPayload<P, E, R>: PayloadAction
{
}

impl<P, E, R: ReturnsValue> ReturnsValue for DynamicPayload<P, E, R> where
    DynamicPayload<P, E, R>: PayloadAction
{
}

impl<P, E, R> DynamicPayload<P, E, R>
where
    P: PayloadAction<ReturnType = E>,
    R: PayloadAction,
{
    pub fn new(prev_payload: P, action: fn(E) -> R) -> DynamicPayload<P, E, R> {
        DynamicPayload {
            prev_payload,
            action,
        }
    }
}

impl<P, E, T> PayloadAction for DynamicPayload<P, E, T>
where
    P: PayloadAction<ReturnType = E>,
    T: PayloadAction,
{
    type ReturnType = T::ReturnType;

    async fn execute<T1: PipeRead + PipeWrite + Unpin>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        let new_payload = (self.action)(result);
        new_payload.execute(pipe).await
    }
}
