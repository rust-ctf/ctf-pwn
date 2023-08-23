use super::*;

use tokio::{
    io::Result,
    net::{tcp::*, *},
};

type TcpStreamPipe = Pipe<OwnedReadHalf, OwnedWriteHalf>;

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
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
