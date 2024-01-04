use crate::io::{
    CacheReader, NcursesTerminalBridge, Pipe, PipeReadExt, PipeWriteExt,
    ShellTerminalBridge, TerminalBridge, TimeoutReader, TimeoutWriter,
};

use tokio::io::{
    AsyncRead, AsyncWrite,
};

impl<T, R, W> PipeInteractiveExt<R, W> for T
where
    T: AsyncReadWriteSplit<R, W> + Unpin + PipeReadExt + PipeWriteExt,
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
}

pub trait AsyncReadWriteSplit<R, W> {
    unsafe fn split_read_write(&mut self) -> (&mut R, &mut W);
}

impl<R, W> AsyncReadWriteSplit<CacheReader<TimeoutReader<R>>, TimeoutWriter<W>> for Pipe<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    unsafe fn split_read_write(
        &mut self,
    ) -> (&mut CacheReader<TimeoutReader<R>>, &mut TimeoutWriter<W>) {
        (&mut self.reader, &mut self.writer)
    }
}

pub trait PipeInteractiveExt<R, W>: AsyncReadWriteSplit<R, W> + Unpin
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    async fn shell(&mut self) -> Result<(), ()> {
        let (reader, writer) = unsafe { self.split_read_write() };
        ShellTerminalBridge::bridge(reader, writer).await;
        Ok(())
    }

    async fn interactive(&mut self) -> Result<(), ()> {
        let (reader, writer) = unsafe { self.split_read_write() };
        NcursesTerminalBridge::bridge(reader, writer).await;
        Ok(())
    }
}
