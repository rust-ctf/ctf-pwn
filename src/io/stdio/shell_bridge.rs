use crate::io::stdio::{is_stop_terminal, is_terminate_process, TerminalBridge, TerminalResult};
use std::io::stdout;
use std::io::ErrorKind::TimedOut;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use ascii::AsciiChar::s;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo};
use crossterm::event::{
    DisableBracketedPaste, EnableBracketedPaste, KeyCode, KeyEvent, KeyEventKind,
};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::*;
use tokio::join;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct ShellTerminalBridge {}

struct StdoutState<'a, W: AsyncWrite + Unpin> {
    text: String,
    start_position: (u16, u16),
    cursor_position: (u16, u16),
    //TODO: Handle resize correctly
    current_dimensions: (u16, u16),
    writer: &'a mut W,
    stop_signal: Arc<AtomicBool>,
}

impl<'a, W: AsyncWrite + Unpin> StdoutState<'a, W> {
    pub fn new(writer: &mut W, stop_signal: Arc<AtomicBool>) -> TerminalResult<StdoutState<W>> {
        let cursor_position = cursor::position()?;
        let current_dimensions = terminal::size()?;
        Ok(StdoutState {
            text: String::new(),
            start_position: cursor_position,
            cursor_position,
            current_dimensions,
            writer,
            stop_signal,
        })
    }

    pub async fn insert(&mut self, key_event: KeyEvent) -> TerminalResult<()> {
        if is_terminate_process(key_event) {
            std::process::exit(130); //SIGINT
        }

        if is_stop_terminal(key_event) {
            self.stop_signal.store(true, Ordering::SeqCst);
            return Ok(());
        }

        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char(c) => self.insert_char(c)?,
            KeyCode::Left => self.decrement_cursor()?,
            KeyCode::Right => self.increment_cursor()?,
            KeyCode::Backspace => self.backspace()?,
            KeyCode::Delete => self.del()?,
            KeyCode::Enter => self.send_data().await?,
            KeyCode::Home => self.home()?,
            KeyCode::End => self.end()?,
            _ => {}
        };

        Ok(())
    }

    fn get_cursor_relative_index(&self) -> usize {
        let (start_x, start_y) = self.start_position;
        let (end_x, end_y) = self.cursor_position;
        let (w, _h) = self.current_dimensions;

        let full_lines = if end_y > start_y {
            end_y - start_y - 1
        } else {
            0
        };
        let last_line_chars = if end_y > start_y {
            end_x
        } else {
            end_x - start_x
        };

        full_lines as usize * w as usize + last_line_chars as usize
    }

    fn set_cursor_relative_index(&mut self, index: usize) -> TerminalResult<()> {
        if index > self.text.len() {
            return Ok(());
        }

        let (start_x, start_y) = self.start_position;
        let (w, _) = self.current_dimensions;

        let lines_down = index / w as usize;
        let new_y = start_y + lines_down as u16;

        let new_x = (start_x as usize + index % w as usize) as u16;

        let (final_x, final_y) = if new_x >= w {
            (new_x - w, new_y + 1)
        } else {
            (new_x, new_y)
        };

        self.set_cursor_position(final_x, final_y)
    }

    fn increment_cursor(&mut self) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        if index >= self.text.len() {
            return Ok(());
        }
        self.set_cursor_relative_index(index + 1)
    }

    fn decrement_cursor(&mut self) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        if index <= 0 {
            return Ok(());
        }
        self.set_cursor_relative_index(index - 1)
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) -> TerminalResult<()> {
        self.cursor_position = (x, y);
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn print(&mut self, data: &[u8]) -> TerminalResult<()> {
        self.clear()?;
        terminal::disable_raw_mode().unwrap();
        stdout().write_all(data)?;
        stdout().flush()?;
        terminal::enable_raw_mode().unwrap();
        self.start_position = cursor::position()?;
        self.cursor_position = self.start_position;
        self.redraw()?;
        Ok(())
    }

    pub fn insert_str(&mut self, text: &str) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        self.text.insert_str(index, text);
        self.redraw()?;
        self.set_cursor_relative_index(index + text.len())?;
        println!("\n{text}\n");
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        self.text.insert(index, c);
        self.redraw()?;
        self.set_cursor_relative_index(index + 1)?;
        Ok(())
    }

    fn del(&mut self) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        if index >= self.text.len() {
            return Ok(());
        }
        let _ = self.text.remove(index);
        self.redraw()?;
        self.set_cursor_relative_index(index)?;
        Ok(())
    }

    fn backspace(&mut self) -> TerminalResult<()> {
        let index = self.get_cursor_relative_index();
        if index <= 0 {
            return Ok(());
        }
        let _ = self.text.remove(index - 1);
        self.redraw()?;
        self.set_cursor_relative_index(index - 1)?;
        Ok(())
    }

    pub fn home(&mut self) -> TerminalResult<()> {
        self.set_cursor_relative_index(0)
    }

    pub fn end(&mut self) -> TerminalResult<()> {
        self.set_cursor_relative_index(self.text.len())
    }

    pub async fn send_data(&mut self) -> TerminalResult<()> {

        self.end()?;
        let (_ ,y) = cursor::position()?;
        self.set_cursor_position(0, y + 1);
        self.start_position = self.cursor_position;
        let mut text = self.text.clone();
        self.text.clear();
        text.push('\n');

        self.writer.write_all(text.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    fn clear(&mut self) -> TerminalResult<()> {
        let (start_x, start_y) = self.start_position;
        self.set_cursor_position(start_x, start_y)?;

        execute!(
            stdout(),
            MoveTo(start_x, start_y),
            Clear(ClearType::FromCursorDown)
        )?;
        Ok(())
    }

    pub fn redraw(&mut self) -> TerminalResult<()> {
        self.clear()?;
        let (start_x, start_y) = self.start_position;

        execute!(stdout(), MoveTo(start_x, start_y), Print(&self.text))?;
        Ok(())
    }
}

async fn read_task<R>(
    reader: &mut R,
    stop_signal: Arc<AtomicBool>,
    sender: Sender<Vec<u8>>,
) -> TerminalResult<()>
where
    R: AsyncRead + Send + Unpin,
{
    let mut buffer = [0; 1024];
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        let n = match reader.read(&mut buffer).await {
            Ok(n) if n == 0 => continue,
            Ok(n) => n,
            Err(e) if e.kind() == TimedOut => continue,
            Err(e) => return Err(e.into()),
        };

        sender.send(buffer[..n].to_vec()).await?
    }
    Ok(())
}

async fn write_task<W>(
    writer: &mut W,
    stop_signal: Arc<AtomicBool>,
    mut receiver: Receiver<Vec<u8>>,
) -> TerminalResult<()>
where
    W: AsyncWrite + Send + Unpin,
{
    let mut stdout = StdoutState::new(writer, stop_signal.clone())?;

    loop {
        if stop_signal.load(Ordering::SeqCst) {
            return Ok(());
        }

        match receiver.try_recv() {
            Ok(data) => {
                stdout.print(&data)?;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
        }

        if let Ok(true) = event::poll(Duration::from_millis(0)) {
            match event::read() {
                Ok(event::Event::Key(key_event)) => {
                    stdout.insert(key_event).await?;
                }
                Ok(event::Event::Resize(_width, _height)) => {
                    //TODO: recalculate cursor after resize
                }
                Ok(event::Event::Paste(text)) => {
                    stdout.insert_str(&text)?;
                }
                _ => {}
            }
        }
    }
}

impl TerminalBridge for ShellTerminalBridge {
    async fn bridge<R: AsyncRead + Send + Unpin, W: AsyncWrite + Send + Unpin>(
        reader: &mut R,
        writer: &mut W,
    ) {
        let reader_ptr = reader as *mut R as usize;
        let writer_ptr = writer as *mut W as usize;

        let (rx, tx) = channel(100);

        terminal::enable_raw_mode().unwrap();
        let _ = execute!(stdout(), EnableBlinking, EnableBracketedPaste);

        let stop_signal = Arc::new(AtomicBool::new(false));
        let read_stop_signal = stop_signal.clone();

        let reader_task = tokio::spawn(async move {
            let reader_ptr = reader_ptr as *mut R;
            let reader = unsafe { &mut *reader_ptr };
            let _ = read_task(reader, read_stop_signal.clone(), rx).await;
            read_stop_signal.store(true, Ordering::SeqCst);
        });

        let writer_task = tokio::spawn(async move {
            let writer_ptr = writer_ptr as *mut W;
            let writer = unsafe { &mut *writer_ptr };
            let _ = write_task(writer, stop_signal.clone(), tx).await;
            stop_signal.store(true, Ordering::SeqCst);
        });

        let _ = join!(reader_task, writer_task);

        let _ = execute!(stdout(), DisableBlinking, DisableBracketedPaste);

        terminal::disable_raw_mode().unwrap();
    }
}
