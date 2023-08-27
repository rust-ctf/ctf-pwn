use super::*;

use tokio::io::AsyncRead;
use tokio::io::{split, ReadHalf, WriteHalf};

impl<T: AsyncRead + AsyncWrite> From<T> for Pipe<ReadHalf<T>, WriteHalf<T>> {
    fn from(value: T) -> Self {
        let (read_stream, write_stream) = split(value);
        Pipe::new(read_stream, write_stream)
    }
}
