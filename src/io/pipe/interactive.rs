use crate::io::cache::HasCache;
use crate::io::pipe::write::PipeWriteExt;
use crate::io::timeout::HasTimeout;
use crate::io::{Pipe, PipeReadExt};
use std::io::ErrorKind::TimedOut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{
    self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader,
};
use tokio::join;

pub trait AsyncReadWriteSplit<R, W> {
    unsafe fn split_read_write(&mut self) -> (*mut R, *mut W);
}

impl<R, W> AsyncReadWriteSplit<R, W> for Pipe<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    unsafe fn split_read_write(&mut self) -> (*mut R, *mut W) {
        let reader_ptr = self.reader.get_mut() as *mut R;
        let writer_ptr = self.writer.get_mut() as *mut W;
        (reader_ptr, writer_ptr)
    }
}

//TODO: Add error messages if read_task or write_task fail
async fn read_task<R>(reader: &mut R, stop_signal: Arc<AtomicBool>)
where
    R: AsyncRead + Unpin + Send + HasTimeout + PipeReadExt,
{
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024];
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        let n = match reader.read(&mut buffer).await {
            Ok(n) if n == 0 => continue,
            Ok(n) => n,
            Err(e) if e.kind() == TimedOut => continue,
            Err(_) => {
                stop_signal.store(true, Ordering::SeqCst);
                break;
            }
        };
        match stdout.write_all(&buffer[..n]).await {
            Ok(_) => {}
            Err(_) => {
                stop_signal.store(true, Ordering::SeqCst);
                break;
            }
        }
    }
}

async fn write_task<W>(writer: &mut W, stop_signal: Arc<AtomicBool>)
where
    W: AsyncWrite + Unpin + Send + HasTimeout + PipeWriteExt,
{
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        if stop_signal.load(Ordering::SeqCst) {
            break;
        }

        let line = match lines.next_line().await {
            Err(_) | Ok(None) => {
                stop_signal.store(true, Ordering::SeqCst);
                break;
            }
            Ok(Some(line)) => line,
        };
        match writer.write_line(&line).await {
            Ok(_) => {}
            Err(_) => {
                stop_signal.store(true, Ordering::SeqCst);
                break;
            }
        }
    }
}

pub trait PipeInteractiveExt<R, W>:
    AsyncReadWriteSplit<R, W> + HasCache + HasTimeout + Unpin
where
    R: AsyncRead + Unpin + Send + HasTimeout + PipeReadExt,
    W: AsyncWrite + Unpin + Send + HasTimeout + PipeWriteExt,
{
    async fn shell(&mut self) -> Result<(), ()> {
        let (reader_ptr, writer_ptr) = unsafe { self.split_read_write() };

        let stop_signal = Arc::new(AtomicBool::new(false));

        let reader = unsafe { &mut *reader_ptr };
        let writer = unsafe { &mut *writer_ptr };

        let reader_task = read_task(reader, stop_signal.clone());
        let writer_task = write_task(writer, stop_signal);

        join!(reader_task, writer_task);

        Ok(())
    }
}
