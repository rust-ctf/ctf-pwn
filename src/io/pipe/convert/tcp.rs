use crate::io::Pipe;
use tokio::{
    io::Result,
    net::{tcp::*, *},
};

pub type TcpPipe = Pipe<OwnedReadHalf, OwnedWriteHalf>;

impl TcpPipe {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpPipe> {
        let stream = TcpStream::connect(addr).await?;
        Ok(stream.into())
    }
}

impl From<TcpStream> for TcpPipe {
    fn from(value: TcpStream) -> Self {
        let (read_stream, write_stream) = value.into_split();
        Pipe::new(read_stream, write_stream)
    }
}
