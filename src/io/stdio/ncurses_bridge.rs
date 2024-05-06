use crate::io::stdio::{is_stop_terminal, is_terminate_process, TerminalBridge, TerminalResult};
use crate::io::{ansi, TerminalError};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::*;
use std::io::ErrorKind::TimedOut;
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use crossterm::terminal::*;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::join;
use tokio::task::JoinError;

pub struct NcursesTerminalBridge {}

async fn read_task<R>(reader: &mut R, stop_signal: Arc<AtomicBool>) -> TerminalResult<()>
where
    R: AsyncRead + Send + Unpin,
{
    let mut buffer = [0; 1024];
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        let n = match reader.read(&mut buffer).await {
            Ok(n) if n == 0 => break, //EOF
            Ok(n) => n,
            Err(e) if e.kind() == TimedOut => continue,
            Err(e) => return Err(e.into()),
        };

        let mut stdout = tokio::io::stdout();
        terminal::disable_raw_mode().unwrap();
        stdout.write_all(&buffer[..n]).await?;
        stdout.flush().await?;
        terminal::enable_raw_mode().unwrap();
    }
    Ok(())
}

async fn write_ansi_command<W: AsyncWrite + Send + Unpin, T: Command>(
    writer: &mut W,
    command: T,
) -> Result<usize, TerminalError> {
    let mut ansi_command = String::new();
    command.write_ansi(&mut ansi_command)?;
    let size = writer.write(ansi_command.as_bytes()).await?;
    Ok(size)
}

async fn send_key<W>(
    writer: &mut W,
    stop_signal: Arc<AtomicBool>,
    key_event: KeyEvent,
) -> TerminalResult<()>
where
    W: AsyncWrite + Send + Unpin,
{
    if is_terminate_process(key_event) {
        return Err(TerminalError::Terminate);
    }

    if is_stop_terminal(key_event) {
        stop_signal.store(true, Ordering::SeqCst);
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
        return Ok(());
    }

    match key_event.code {
        KeyCode::Char(c) => {
            let mut buf = [0; 4]; // UTF-8 encoding of a char may use up to 4 bytes
            let bytes = c.encode_utf8(&mut buf);
            writer.write_all(bytes.as_bytes()).await?;
        }
        KeyCode::Left => {
            write_ansi_command(writer, ansi::Left).await?;
        }
        KeyCode::Right => {
            write_ansi_command(writer, ansi::Right).await?;
        }
        KeyCode::Up => {
            write_ansi_command(writer, ansi::Up).await?;
        }
        KeyCode::Down => {
            write_ansi_command(writer, ansi::Down).await?;
        }
        KeyCode::Delete => {
            write_ansi_command(writer, ansi::Del).await?;
        }
        KeyCode::Backspace => {
            write_ansi_command(writer, ansi::Backspace).await?;
        }
        KeyCode::Esc => {
            write_ansi_command(writer, ansi::Esc).await?;
        }
        KeyCode::Enter => {
            write_ansi_command(writer, ansi::Enter).await?;
        }
        KeyCode::Home => {
            write_ansi_command(writer, ansi::Home).await?;
        }
        KeyCode::End => {
            write_ansi_command(writer, ansi::End).await?;
        }
        KeyCode::PageUp => {
            write_ansi_command(writer, ansi::PgUp).await?;
        }
        KeyCode::PageDown => {
            write_ansi_command(writer, ansi::PgDn).await?;
        }
        KeyCode::F(no) => {
            write_ansi_command(writer, ansi::F(no)).await?;
        }
        _ => {}
    };

    writer.flush().await?;

    Ok(())
}

async fn write_task<W>(writer: &mut W, stop_signal: Arc<AtomicBool>) -> TerminalResult<()>
where
    W: AsyncWrite + Send + Unpin,
{
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            return Ok(());
        }

        if let Ok(true) = event::poll(Duration::from_millis(0)) {
            match event::read() {
                Ok(event::Event::Key(key_event)) => {
                    send_key(writer, stop_signal.clone(), key_event).await?;
                }
                _ => {}
            }
        }
    }
}

impl TerminalBridge for NcursesTerminalBridge {
    async fn bridge<R: AsyncRead + Send + Unpin, W: AsyncWrite + Send + Unpin>(
        reader: &mut R,
        writer: &mut W,
    ) {
        let reader_ptr = reader as *mut R as usize;
        let writer_ptr = writer as *mut W as usize;

        execute!(stdout(), EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        let stop_signal = Arc::new(AtomicBool::new(false));
        let read_stop_signal = stop_signal.clone();

        let reader_task = tokio::spawn(async move {
            let reader_ptr = reader_ptr as *mut R;
            let reader = unsafe { &mut *reader_ptr };
            let res = read_task(reader, read_stop_signal.clone()).await;
            read_stop_signal.store(true, Ordering::SeqCst);
            res
        });

        let writer_task = tokio::spawn(async move {
            let writer_ptr = writer_ptr as *mut W;
            let writer = unsafe { &mut *writer_ptr };
            let res = write_task(writer, stop_signal.clone()).await;
            stop_signal.store(true, Ordering::SeqCst);
            res
        });

        let (read_res, write_res) = join!(reader_task, writer_task);

        terminal::disable_raw_mode().unwrap();

        execute!(stdout(), LeaveAlternateScreen).unwrap();

        if let Ok(Err(TerminalError::Terminate)) = read_res
        {
            std::process::exit(130); //SIGINT
        }
        else if  let Ok(Err(TerminalError::Terminate)) = write_res
        {
            std::process::exit(130); //SIGINT
        }
    }
}
