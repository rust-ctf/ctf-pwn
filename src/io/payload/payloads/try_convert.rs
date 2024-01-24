use crate::io::*;

#[derive(Clone)]
pub struct TryConvert<P, E, R> {
    prev_payload: P,
    action: fn(E) -> Result<R, PipeError>,
}

impl<P, E, R> Buildable for TryConvert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> Sendable for TryConvert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> Readable for TryConvert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> ReturnsValue for TryConvert<P, E, R> where Self: PayloadAction {}

impl<P, E, R> TryConvert<P, E, R>
where
    P: PayloadAction<ReturnType = E>,
{
    pub fn new(prev_payload: P, action: fn(E) -> Result<R, PipeError>) -> TryConvert<P, E, R> {
        TryConvert {
            prev_payload,
            action,
        }
    }
}

impl<P, E, T> PayloadAction for TryConvert<P, E, T>
where
    P: PayloadAction<ReturnType = E>, E: Clone, T: Clone,
{
    type ReturnType = T;

    async fn execute<T1: PipeRead + PipeWrite + Unpin>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        (self.action)(result)
    }
}
