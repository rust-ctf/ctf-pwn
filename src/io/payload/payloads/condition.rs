use crate::io::*;

#[derive(Clone)]
pub struct Condition<P, F> {
    prev_payload: P,
    action: F,
}

impl<P, F> Buildable for Condition<P, F> where Self: PayloadAction {}
impl<P, F> Sendable for Condition<P, F> where Self: PayloadAction {}
impl<P, F> Readable for Condition<P, F> where Self: PayloadAction {}
impl<P, F> ReturnsValue for Condition<P, F> where Self: PayloadAction {}

impl<P, F> Condition<P, F>
where
    P: PayloadAction,
    F: Fn(&P::ReturnType) -> bool + Clone
{
    pub fn new(prev_payload: P, action: F) -> Condition<P, F>
    {
        Condition {
            prev_payload,
            action,
        }
    }
}

impl<P, F> PayloadAction for Condition<P, F>
where
    P: PayloadAction,
    F: Fn(&P::ReturnType) -> bool + Clone
{
    type ReturnType = P::ReturnType;

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
