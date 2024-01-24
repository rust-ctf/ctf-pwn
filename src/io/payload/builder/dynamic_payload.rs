use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::DynamicPayload;
use crate::io::*;

impl<T: ReturnsValue, A> PayloadBuilder<T, A> {
    pub fn payload<F, R>(
        self,
        action: F,
    ) -> PayloadBuilder<DynamicPayload<T, F, R>, A>
    where
        F: Fn(T::ReturnType) -> R + Clone,
        R: PayloadAction,
    {
        PayloadBuilder::from(DynamicPayload::<T,F,R>::new(self.payload, action))
    }
}
