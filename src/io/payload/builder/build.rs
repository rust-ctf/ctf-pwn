use crate::io::{Buildable, PayloadBuilder};

impl<T: Buildable, A> PayloadBuilder<T, A> {
    pub fn build(self) -> T {
        self.payload
    }
}
