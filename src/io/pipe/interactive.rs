use crate::io::{
    CacheReader, NcursesTerminalBridge, Pipe, PipeError, PipeRead, PipeWriteExt,
    ShellTerminalBridge, TerminalBridge,
};
use tokio::io::{AsyncRead, AsyncWrite};

impl<T, R, W> PipeInteractiveExt<R, W> for T
where
    T: AsyncReadWriteSplit<R, W> + Unpin + PipeRead + PipeWriteExt,
    R: AsyncRead + Send + Unpin,
    W: AsyncWrite + Send + Unpin,
{
}

pub trait AsyncReadWriteSplit<R, W> {
    unsafe fn split_read_write(&mut self) -> (&mut R, &mut W);
}

impl<R, W> AsyncReadWriteSplit<CacheReader<R>, W> for Pipe<R, W>
where
    R: AsyncRead + Send + Unpin,
    W: AsyncWrite + Send + Unpin,
{
    unsafe fn split_read_write(&mut self) -> (&mut CacheReader<R>, &mut W) {
        (&mut self.reader, &mut self.writer)
    }
}

pub trait PipeInteractiveExt<R, W>: AsyncReadWriteSplit<R, W> + Unpin
where
    R: AsyncRead + Send + Unpin,
    W: AsyncWrite + Send + Unpin,
{
    async fn interactive_shell(&mut self) -> Result<(), PipeError> {
        let (reader, writer) = unsafe { self.split_read_write() };
        ShellTerminalBridge::bridge(reader, writer).await;
        Ok(())
    }

    async fn interactive_ansi(&mut self) -> Result<(), PipeError> {
        let (reader, writer) = unsafe { self.split_read_write() };
        NcursesTerminalBridge::bridge(reader, writer).await;
        Ok(())
    }
}
