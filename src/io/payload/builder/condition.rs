use crate::io::payload::builder::PayloadBuilder;
use crate::io::{Condition, PayloadAction, ReturnsValue};

impl<T: ReturnsValue, A> PayloadBuilder<T, A>
{
    pub fn condition(self, action: fn(&T::ReturnType) -> bool) -> PayloadBuilder<Condition<T, T::ReturnType>, A>
    {
        PayloadBuilder::from(Condition::<T, T::ReturnType>::new(self.payload, action))
    }
}