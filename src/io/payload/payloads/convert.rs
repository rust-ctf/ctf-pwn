use crate::io::*;

#[derive(Clone)]
pub struct Convert<P, E, R> {
    prev_payload: P,
    action: fn(E) -> R,
}

impl<P, E, R> Buildable for Convert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> Sendable for Convert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> Readable for Convert<P, E, R> where Self: PayloadAction {}
impl<P, E, R> ReturnsValue for Convert<P, E, R> where Self: PayloadAction {}

impl<P, E, R> Convert<P, E, R>
where
    P: PayloadAction<ReturnType = E>,
{
    pub fn new(prev_payload: P, action: fn(E) -> R) -> Convert<P, E, R> {
        Convert {
            prev_payload,
            action,
        }
    }
}

impl<P, E, T> PayloadAction for Convert<P, E, T>
where
    P: PayloadAction<ReturnType = E>, E: Clone, T:Clone
{
    type ReturnType = T;

    async fn execute<T1: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        Ok((self.action)(result))
    }
}
