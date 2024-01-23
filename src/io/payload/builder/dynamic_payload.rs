use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::DynamicPayload;
use crate::io::*;

impl<T: ReturnsValue, A> PayloadBuilder<T, A> {
    pub fn payload<R>(
        self,
        action: fn(T::ReturnType) -> R,
    ) -> PayloadBuilder<DynamicPayload<T, T::ReturnType, R>, A>
    where
        R: PayloadAction,
    {
        PayloadBuilder::from(DynamicPayload::new(self.payload, action))
    }
}
