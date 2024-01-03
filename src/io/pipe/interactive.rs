use crate::io::{
    CacheReader, HasCache, HasTimeout, NcursesTerminalBridge, Pipe, PipeReadExt, PipeWriteExt,
    ShellTerminalBridge, TerminalBridge, TimeoutReader, TimeoutWriter,
};
use ascii::{AsciiChar, AsciiString};
use crossterm::cursor::*;
use crossterm::cursor::{position, DisableBlinking, RestorePosition, SavePosition};
use crossterm::event::KeyEventKind;
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::*;
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::io::ErrorKind::TimedOut;
use std::ptr::write;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{
    self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader,
};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::sleep;
use tokio::{join, task};

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
