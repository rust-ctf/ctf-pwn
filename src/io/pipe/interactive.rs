use crate::io::{CacheReader, HasCache, HasTimeout, Pipe, PipeReadExt, PipeWriteExt, TimeoutReader, TimeoutWriter};
use std::io::ErrorKind::TimedOut;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::{join, task};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::ptr::write;
use std::time::Duration;
use ascii::{AsciiChar, AsciiString};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::time::sleep;
use crossterm::*;
use crossterm::cursor::{DisableBlinking, position, RestorePosition, SavePosition};
use crossterm::terminal::*;
use crossterm::cursor::*;
use crossterm::event::KeyEventKind;
use crossterm::style::*;

impl<T, R, W> PipeInteractiveExt<R, W> for T
where
    T: AsyncReadWriteSplit<R, W> + Unpin + PipeReadExt + PipeWriteExt,
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
}

pub trait AsyncReadWriteSplit<R, W> {
    unsafe fn split_read_write(&mut self) -> (*mut R, *mut W);
}

impl<R, W> AsyncReadWriteSplit<CacheReader<TimeoutReader<R>>, TimeoutWriter<W>> for Pipe<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    unsafe fn split_read_write(&mut self) -> (*mut CacheReader<TimeoutReader<R>>, *mut TimeoutWriter<W>) {
        let reader_ptr = self.reader.borrow_mut() as *mut CacheReader<TimeoutReader<R>>;
        let writer_ptr = self.writer.borrow_mut() as *mut TimeoutWriter<W>;
        (reader_ptr, writer_ptr)
    }
}

//TODO: Add error messages if read_task or write_task fail
async fn read_task<R>(reader: &mut R, stop_signal: Arc<AtomicBool>, sender: Sender<Vec<u8>>)
where
    R: AsyncRead + Unpin + Send,
{
    let mut buffer = [0; 1024];
    loop {
        if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        let n = match reader.read(&mut buffer).await {
            Ok(n) if n == 0 => continue,
            Ok(n) => n,
            Err(e) if e.kind() == TimedOut => {
                continue
            },
            Err(_) => {
                stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        };

        match sender.send(buffer[..n].to_vec()).await {
            Ok(_) => {}
            Err(_) => {
                stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    }
}

async fn write_task<W>(writer: &mut W, stop_signal: Arc<AtomicBool>, mut receiver: Receiver<Vec<u8>>)
where
    W: AsyncWrite + Unpin + Send,
{
    execute!(std::io::stdout(), SavePosition);

    let mut stdout = io::stdout();

    let mut cache = String::new();
    loop {
        if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        match receiver.try_recv()
        {
            Ok(data) => {
                match stdout.write_all(&data).await
                {
                    Ok(_) => {
                        let _ = stdout.flush().await;
                        execute!(std::io::stdout(), SavePosition);
                    }
                    Err(_) => {
                        stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);
                        break;
                    }
                }
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);
                break;
            },
        }

        if let Ok(true) = event::poll(Duration::from_millis(0)) {
            match event::read() {
                Ok(event::Event::Key(key_event))  if key_event.kind == KeyEventKind::Press => {
                   handle_key_event(key_event, &mut cache, writer).await;
                },
                Ok(event::Event::Resize(width, height)) => {},
                Ok(event::Event::Paste(text)) =>
                {
                    cache.push_str(&text);
                    clear_stdout().await;
                    execute!(std::io::stdout(), Print(&cache));
                }
                _ => {},
            }
        }
    }
}

async fn clear_stdout()
{
    let (x,y) = cursor::position().unwrap();
    execute!(std::io::stdout(), RestorePosition).unwrap();

    let (old_x,old_y) = cursor::position().unwrap();

    if y >= old_y
    {
        execute!(std::io::stdout(), Clear(ClearType::UntilNewLine)).unwrap();
        for line in old_y + 1..=y {
            execute!(std::io::stdout(), MoveTo(0, line), Clear(ClearType::CurrentLine)).unwrap();
        }
        execute!(std::io::stdout(), MoveTo(old_x, old_y)).unwrap();
    }

    execute!(std::io::stdout(), SavePosition).unwrap();
}

async fn handle_key_event<W>(key_event: event::KeyEvent, cache: &mut String, writer: &mut W)
    where
        W: AsyncWrite + Unpin + Send,
{
    clear_stdout().await;
    match key_event.code {
        event::KeyCode::Char(c) => {
            cache.push(c);
        },
        event::KeyCode::Backspace => {
            let _ = cache.pop();
        },
        event::KeyCode::Enter => {
            writer.write(cache.as_bytes()).await.unwrap();
            let _ = writer.flush().await;
            execute!(std::io::stdout(), Print(&cache));
            cache.clear();
            return;
        },
        event::KeyCode::Left => {
            // Move cursor left...
        },
        event::KeyCode::Right => {
            // Move cursor right...
        },
        event::KeyCode::Delete => {
            // Handle delete...
        }
        _ => {}
    }
    execute!(std::io::stdout(), Print(cache));
}

pub trait PipeInteractiveExt<R, W>: AsyncReadWriteSplit<R, W> + Unpin
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    async fn shell(&mut self) -> Result<(), ()> {
        let (reader_ptr, writer_ptr) = unsafe { self.split_read_write() };

        let stop_signal = Arc::new(AtomicBool::new(false));

        let (rx,tx) = channel(100);

        let reader_ptr = reader_ptr as usize;
        let writer_ptr = writer_ptr as usize;


        terminal::enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnableBlinking);

        let read_stop_signal = stop_signal.clone();
        let reader_task = tokio::spawn(async move {
            let reader_ptr = unsafe { reader_ptr as *mut R};
            let reader = unsafe { &mut *reader_ptr };
            read_task(reader, read_stop_signal.clone(), rx).await
        });

        let writer_task = tokio::spawn(async move {
            let writer_ptr = unsafe { writer_ptr as *mut W};
            let writer = unsafe { &mut *writer_ptr };
            write_task(writer, stop_signal, tx).await
        });

        join!(reader_task, writer_task);

        execute!(std::io::stdout(), DisableBlinking);

        terminal::disable_raw_mode().unwrap();

        Ok(())
    }
}
