use std::marker::PhantomData;
use crate::io::*;

#[derive(Clone)]
pub struct TryConvert<P, F, R> {
    prev_payload: P,
    action: F,
    _phantom: PhantomData<R>,
}

impl<P, F, R> Buildable for TryConvert<P, F, R> where Self: PayloadAction {}
impl<P, F, R> Sendable for TryConvert<P, F, R> where Self: PayloadAction {}
impl<P, F, R> Readable for TryConvert<P, F, R> where Self: PayloadAction {}
impl<P, F, R> ReturnsValue for TryConvert<P, F, R> where Self: PayloadAction {}

impl<P, F, R> TryConvert<P, F, R>
where
    P: ReturnsValue,
    F: Fn(P::ReturnType) -> Result<R, PipeError> + Clone,
    R: Clone
{
    pub fn new(prev_payload: P, action: F) -> TryConvert<P, F, R>
    {
        TryConvert {
            prev_payload,
            action,
            _phantom: PhantomData::default()
        }
    }
}

impl<P, F, R> PayloadAction for TryConvert<P, F, R>
where
    P: ReturnsValue,
    F: Fn(P::ReturnType) -> Result<R, PipeError> + Clone,
    R: Clone,
{
    type ReturnType = R;

    async fn execute<T1: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        (self.action)(result)
    }
}
