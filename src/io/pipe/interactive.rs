use crate::io::{NcursesTerminalBridge, PipeError, ShellTerminalBridge, TerminalBridge};
use tokio::io::{AsyncRead, AsyncWrite, split};

impl<T: AsyncRead + AsyncWrite + Unpin + Send + ?Sized> PipeInteractiveExt for T {}

pub trait PipeInteractiveExt: AsyncRead + AsyncWrite + Unpin + Send
{
    async fn interactive_shell(&mut self) -> Result<(), PipeError> {
        let (mut reader, mut writer) = split(self);
        ShellTerminalBridge::bridge(&mut reader, &mut writer).await;
        Ok(())
    }

    async fn interactive_ansi(&mut self) -> Result<(), PipeError> {
        let (mut reader, mut writer) = split(self);
        NcursesTerminalBridge::bridge(&mut reader, &mut writer).await;
        Ok(())
    }
}
