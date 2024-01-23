use crate::io::payload::builder::PayloadBuilder;
use crate::io::{Convert, PayloadAction, PipeError, ReturnsValue, TryConvert};

impl<T: ReturnsValue, A> PayloadBuilder<T, A>
{
    pub fn convert<R>(self, action: fn(T::ReturnType) -> R) -> PayloadBuilder<Convert<T, T::ReturnType, R>, A>
    {
        PayloadBuilder::from(Convert::<T, T::ReturnType, R>::new(self.payload, action))
    }

    pub fn try_convert<R>(self, action: fn(T::ReturnType) -> Result<R, PipeError>) -> PayloadBuilder<TryConvert<T, T::ReturnType, R>, A>
    {
        PayloadBuilder::from(TryConvert::<T, T::ReturnType, R>::new(self.payload, action))
    }
}