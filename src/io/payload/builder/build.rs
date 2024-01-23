use crate::io::payload::builder::PayloadBuilder;
use crate::io::*;

impl<T: Buildable, A> PayloadBuilder<T, A> {
    pub fn build(self) -> T {
        self.payload
    }
}
