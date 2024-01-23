use crate::io::payload::payloads::*;
use crate::io::{PayloadAction, PayloadBuilder, Readable, ReturnsValue};
use std::fmt::*;

impl<T: ReturnsValue, A> PayloadBuilder<T, A> {
    pub fn print_bytes(self) -> PayloadBuilder<Print<T, FmtBytes>, A>
    where
        T: PayloadAction,
        T::ReturnType: AsRef<[u8]>,
    {
        PayloadBuilder::from(Print::<T, FmtBytes>::from(self.payload))
    }

    pub fn print(self) -> PayloadBuilder<Print<T, FmtDefault>, A>
    where
        T: PayloadAction,
        T::ReturnType: Display,
    {
        PayloadBuilder::from(Print::<T, FmtDefault>::from(self.payload))
    }

    pub fn print_debug(self) -> PayloadBuilder<Print<T, FmtDebug>, A>
    where
        T: PayloadAction,
        T::ReturnType: Debug,
    {
        PayloadBuilder::from(Print::<T, FmtDebug>::from(self.payload))
    }

    pub fn print_lower_hex(self) -> PayloadBuilder<Print<T, FmtLowerHex>, A>
    where
        T: PayloadAction,
        T::ReturnType: LowerHex,
    {
        PayloadBuilder::from(Print::<T, FmtLowerHex>::from(self.payload))
    }

    pub fn print_upper_hex(self) -> PayloadBuilder<Print<T, FmtUpperHex>, A>
    where
        T: PayloadAction,
        T::ReturnType: UpperHex,
    {
        PayloadBuilder::from(Print::<T, FmtUpperHex>::from(self.payload))
    }

    pub fn print_octal(self) -> PayloadBuilder<Print<T, FmtOctal>, A>
    where
        T: PayloadAction,
        T::ReturnType: Octal,
    {
        PayloadBuilder::from(Print::<T, FmtOctal>::from(self.payload))
    }

    pub fn print_binary(self) -> PayloadBuilder<Print<T, FmtBinary>, A>
    where
        T: PayloadAction,
        T::ReturnType: Binary,
    {
        PayloadBuilder::from(Print::<T, FmtBinary>::from(self.payload))
    }

    pub fn print_pointer(self) -> PayloadBuilder<Print<T, FmtPointer>, A>
    where
        T: PayloadAction,
        T::ReturnType: Pointer,
    {
        PayloadBuilder::from(Print::<T, FmtPointer>::from(self.payload))
    }

    pub fn print_lower_exp(self) -> PayloadBuilder<Print<T, FmtLowerExp>, A>
    where
        T: PayloadAction,
        T::ReturnType: LowerExp,
    {
        PayloadBuilder::from(Print::<T, FmtLowerExp>::from(self.payload))
    }

    pub fn print_upper_exp(self) -> PayloadBuilder<Print<T, FmtUpperExp>, A>
    where
        T: PayloadAction,
        T::ReturnType: UpperExp,
    {
        PayloadBuilder::from(Print::<T, FmtUpperExp>::from(self.payload))
    }
}
