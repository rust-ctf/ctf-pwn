use std::marker::PhantomData;
use crate::io::*;

#[derive(Clone)]
pub struct Convert<P, F, E> {
    prev_payload: P,
    action: F,
    _phantom: PhantomData<E>,
}

impl<P,F,E> Buildable for Convert<P, F,E> where Self: PayloadAction {}
impl<P, F,E> Sendable for Convert<P, F,E> where Self: PayloadAction {}
impl<P, F,E> Readable for Convert<P, F,E> where Self: PayloadAction {}
impl<P, F,E> ReturnsValue for Convert<P, F,E> where Self: PayloadAction {}

impl<P, F, E> Convert<P, F, E>
where
    P: PayloadAction,
    F: Fn(P::ReturnType) -> E + Clone
{
    pub fn new(prev_payload: P, action: F) -> Convert<P, F, E>
    {
        Convert {
            prev_payload,
            action,
            _phantom: PhantomData::default(),
        }
    }
}

impl<P, F, E> PayloadAction for Convert<P, F, E>
    where
        P: PayloadAction,
        E: Clone,
        F: Fn(P::ReturnType) -> E + Clone
{
    type ReturnType = E;

    async fn execute<T1: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T1,
    ) -> Result<Self::ReturnType, PipeError> {
        let result = self.prev_payload.execute(pipe).await?;
        Ok((self.action)(result))
    }
}
