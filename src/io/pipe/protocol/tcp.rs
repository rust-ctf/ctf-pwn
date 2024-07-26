use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream, ToSocketAddrs,
};

use crate::io::pipe::{OwnedPipe, PipeError};

pub type TcpPipe = OwnedPipe<OwnedReadHalf, OwnedWriteHalf>;

impl TcpPipe {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpPipe, PipeError> {
        let stream = TcpStream::connect(addr).await?;
        Ok(stream.into())
    }
}

impl From<TcpStream> for TcpPipe {
    fn from(value: TcpStream) -> Self {
        let (read_stream, write_stream) = value.into_split();
        OwnedPipe::new(read_stream, write_stream)
    }
}
