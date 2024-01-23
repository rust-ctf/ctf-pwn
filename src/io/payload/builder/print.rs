use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::*;
use crate::io::{PayloadAction, Readable, ReturnsValue};
use std::fmt::*;

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: AsRef<[u8]>,
{
    pub fn print_bytes(self) -> PayloadBuilder<Print<T, FmtBytes>, A> {
        PayloadBuilder::from(Print::<T, FmtBytes>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: Display,
{
    pub fn print(self) -> PayloadBuilder<Print<T, FmtDefault>, A> {
        PayloadBuilder::from(Print::<T, FmtDefault>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: Debug,
{
    pub fn print_debug(self) -> PayloadBuilder<Print<T, FmtDebug>, A> {
        PayloadBuilder::from(Print::<T, FmtDebug>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: LowerHex,
{
    pub fn print_lower_hex(self) -> PayloadBuilder<Print<T, FmtLowerHex>, A> {
        PayloadBuilder::from(Print::<T, FmtLowerHex>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: UpperHex,
{
    pub fn print_upper_hex(self) -> PayloadBuilder<Print<T, FmtUpperHex>, A> {
        PayloadBuilder::from(Print::<T, FmtUpperHex>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: Octal,
{
    pub fn print_octal(self) -> PayloadBuilder<Print<T, FmtOctal>, A> {
        PayloadBuilder::from(Print::<T, FmtOctal>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: Binary,
{
    pub fn print_binary(self) -> PayloadBuilder<Print<T, FmtBinary>, A> {
        PayloadBuilder::from(Print::<T, FmtBinary>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: Pointer,
{
    pub fn print_pointer(self) -> PayloadBuilder<Print<T, FmtPointer>, A> {
        PayloadBuilder::from(Print::<T, FmtPointer>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: LowerExp,
{
    pub fn print_lower_exp(self) -> PayloadBuilder<Print<T, FmtLowerExp>, A> {
        PayloadBuilder::from(Print::<T, FmtLowerExp>::from(self.payload))
    }
}

impl<T, A> PayloadBuilder<T, A>
where
    T: PayloadAction,
    T::ReturnType: UpperExp,
{
    pub fn print_upper_exp(self) -> PayloadBuilder<Print<T, FmtUpperExp>, A> {
        PayloadBuilder::from(Print::<T, FmtUpperExp>::from(self.payload))
    }
}
