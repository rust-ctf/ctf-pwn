use crate::io::*;

#[derive(Clone)]
pub struct Condition<P, E> {
    prev_payload: P,
    action: fn(&E) -> bool,
}

impl<P, E> Buildable for Condition<P, E> where Self: PayloadAction {}
impl<P, E> Sendable for Condition<P, E> where Self: PayloadAction {}
impl<P, E> Readable for Condition<P, E> where Self: PayloadAction {}
impl<P, E> ReturnsValue for Condition<P, E> where Self: PayloadAction {}

impl<P, E> Condition<P, E>
where
    P: PayloadAction<ReturnType = E>,
{
    pub fn new(prev_payload: P, action: fn(&E) -> bool) -> Condition<P, E> {
        Condition {
            prev_payload,
            action,
        }
    }
}

impl<P, E> PayloadAction for Condition<P, E>
where
    P: PayloadAction<ReturnType = E>, E: Clone
{
    type ReturnType = E;

    async fn execute<T1: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        if !(self.action)(&result) {
            return Err(PipeError::ConditionFailed);
        }
        Ok(result)
    }
}
