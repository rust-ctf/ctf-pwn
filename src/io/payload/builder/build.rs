use crate::io::payload::builder::PayloadBuilder;
use crate::io::Buildable;

impl<T: Buildable, A> PayloadBuilder<T, A> {
    pub fn build(self) -> T {
        self.payload
    }
}
