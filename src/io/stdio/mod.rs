mod ncurses_bridge;
mod shell_bridge;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
pub use ncurses_bridge::*;
pub use shell_bridge::*;
use std::io;
use thiserror::Error;

use crate::io::pipe;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;

pub trait TerminalBridge {
    //TODO: if reader or writer does not have timeout this might get stuck
    async fn bridge<R: AsyncRead + Send + Unpin, W: AsyncWrite + Send + Unpin>(
        reader: &mut R,
        writer: &mut W,
    );
}

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("String Send error: {0}")]
    StringSend(#[from] mpsc::error::SendError<String>),

    #[error("Bytes Send error: {0}")]
    BytesSend(#[from] mpsc::error::SendError<Vec<u8>>),

    #[error("Recv error: {0}")]
    Recv(#[from] mpsc::error::TryRecvError),

    #[error("Pipe error: {0}")]
    Pipe(#[from] pipe::PipeError),

    #[error("format error {0}")]
    FmtError(#[from] std::fmt::Error),
}

type TerminalResult<T> = Result<T, TerminalError>;

fn is_ctr_key(key_event: KeyEvent) -> bool {
    key_event.kind == KeyEventKind::Press && key_event.modifiers == KeyModifiers::CONTROL
}

pub(crate) fn is_terminate_process(key_event: KeyEvent) -> bool {
    return is_ctr_key(key_event) && key_event.code == KeyCode::Char('c');
}

pub(crate) fn is_stop_terminal(key_event: KeyEvent) -> bool {
    return is_ctr_key(key_event) && key_event.code == KeyCode::Char('d');
}
