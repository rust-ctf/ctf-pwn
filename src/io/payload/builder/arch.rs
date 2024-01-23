use crate::io::payload::builder::PayloadBuilder;
use crate::io::*;

impl<T: PayloadAction> PayloadBuilder<T, UnknownArch> {
    pub fn x86(self) -> PayloadBuilder<T, X86> {
        PayloadBuilder::from(self.payload)
    }

    pub fn x64(self) -> PayloadBuilder<T, X64> {
        PayloadBuilder::from(self.payload)
    }
}
