// use crate::io::OwnedPipe;
// use tokio::io::{split, ReadHalf, WriteHalf};
// use tokio::io::{AsyncRead, AsyncWrite};

// impl<T: AsyncRead + AsyncWrite> From<T> for OwnedPipe<ReadHalf<T>, WriteHalf<T>> {
//     fn from(value: T) -> Self {
//         let (read_stream, write_stream) = split(value);
//         OwnedPipe::new(read_stream, write_stream)
//     }
// }
