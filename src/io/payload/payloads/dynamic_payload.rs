use std::marker::PhantomData;
use crate::io::*;

#[derive(Clone)]
pub struct DynamicPayload<P, F, R> {
    prev_payload: P,
    action: F,
    _phantom: PhantomData<R>,
}

impl<P, F, R: Buildable> Buildable for DynamicPayload<P, F, R> where
    DynamicPayload<P, F, R>: PayloadAction
{
}

impl<P, F, R: Sendable> Sendable for DynamicPayload<P, F, R> where
    DynamicPayload<P, F, R>: PayloadAction
{
}

impl<P, F, R: Readable> Readable for DynamicPayload<P, F, R> where
    DynamicPayload<P, F, R>: PayloadAction
{
}

impl<P, F, R: ReturnsValue> ReturnsValue for DynamicPayload<P, F, R> where
    DynamicPayload<P, F, R>: PayloadAction
{
}

impl<P, F, R> DynamicPayload<P, F, R>
where
    P: ReturnsValue,
    F: Fn(P::ReturnType) -> R + Copy,
    R: PayloadAction,
{
    pub fn new(prev_payload: P, action: F) -> DynamicPayload<P, F, R>
    {
        DynamicPayload {
            prev_payload,
            action,
            _phantom: PhantomData::default()
        }
    }
}

impl<P, F, R> PayloadAction for DynamicPayload<P, F, R>
where
    P: PayloadAction,
    F: Fn(P::ReturnType) -> R + Clone,
    R: PayloadAction,
{
    type ReturnType = R::ReturnType;

    async fn execute<T1: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        let new_payload = (self.action)(result);
        new_payload.execute(pipe).await
    }
}
