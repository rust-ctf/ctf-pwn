use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream, ToSocketAddrs,
};

use crate::io::pipe::{OwnedPipe, PipeError};

/// Pipe backed by a TCP connection.
pub type TcpPipe = OwnedPipe<(), OwnedReadHalf, OwnedWriteHalf>;

impl TcpPipe {
    /// Connect to a TCP socket address and return a pipe.
    ///
    /// # Errors
    ///
    /// Returns `PipeError` if the TCP connection fails.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, PipeError> {
        let stream = TcpStream::connect(addr).await?;
        Ok(stream.into())
    }
}

impl From<TcpStream> for TcpPipe {
    fn from(value: TcpStream) -> Self {
        let (read_stream, write_stream) = value.into_split();
        Self::new(read_stream, write_stream)
    }
}
