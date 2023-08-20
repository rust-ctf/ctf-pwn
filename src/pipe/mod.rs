pub(crate) mod interactive;
pub(crate) mod numeric;
pub(crate) mod read;
pub(crate) mod write;

pub use crossterm::event::KeyCode;
pub use interactive::*;
pub use numeric::*;
pub use read::*;
pub use write::*;

use std::collections::VecDeque;
use std::io::{Read, Write};

pub struct Pipe<T> {
    pub(crate) read_cache: VecDeque<u8>,
    pub(crate) stream: T,
}

impl<T> Pipe<T> {
    pub fn from(stream: T) -> Pipe<T> {
        Pipe {
            read_cache: VecDeque::new(),
            stream,
        }
    }
}

impl<T> From<T> for Pipe<T>
where
    T: Read + Write, //TODO: use or trait bound so implicit cast works even when its only read or only write
{
    fn from(value: T) -> Self {
        Self::from(value)
    }
}
