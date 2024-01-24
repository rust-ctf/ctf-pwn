use crate::io::payload::builder::PayloadBuilder;
use crate::io::{Condition, PayloadAction, ReturnsValue};

impl<T: ReturnsValue, A: Clone> PayloadBuilder<T, A>
{
    pub fn condition<F>(
        self,
        action: F,
    ) -> PayloadBuilder<Condition<T, F>, A>
        where F: Fn(&T::ReturnType) -> bool + Clone
    {
        PayloadBuilder::from(Condition::<T, F>::new(self.payload, action))
    }
}
