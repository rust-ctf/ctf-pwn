use crate::io::payload::builder::PayloadBuilder;
use crate::io::{Convert, PayloadAction, PipeError, ReturnsValue, TryConvert};

impl<T: ReturnsValue, A> PayloadBuilder<T, A> {
    pub fn convert<F,E>(
        self,
        action: F,
    ) -> PayloadBuilder<Convert<T, F, E>, A>
        where F: Fn(T::ReturnType) -> E + Copy, E:Copy
    {
        PayloadBuilder::from(Convert::<T, F, E>::new(self.payload, action))
    }

    pub fn try_convert<F,E>(
        self,
        action: F,
    ) -> PayloadBuilder<TryConvert<T, F, E>, A>
        where F: Fn(T::ReturnType) -> Result<E, PipeError> + Copy, E:Copy
    {
        PayloadBuilder::from(TryConvert::<T, F, E>::new(self.payload, action))
    }
}
