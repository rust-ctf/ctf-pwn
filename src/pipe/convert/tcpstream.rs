use super::*;

use tokio::{
    io::Result,
    net::{tcp::*, *},
};

pub type TcpStreamPipe = Pipe<OwnedReadHalf, OwnedWriteHalf>;

impl TcpStreamPipe {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStreamPipe> {
        let stream = TcpStream::connect(addr).await?;
        Ok(stream.into())
    }
}

impl From<TcpStream> for TcpStreamPipe {
    fn from(value: TcpStream) -> Self {
        let (read_stream, write_stream) = value.into_split();
        Pipe::new(read_stream, write_stream)
    }
}
