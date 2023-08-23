mod convert;
mod io;

use std::{time::Duration};

pub use convert::*;
pub use io::*;

use tokio::{io::*, sync::Mutex};

pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(1);
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(1);

pub struct Pipe<R: AsyncRead, W: AsyncWrite>
{
    read_stream: Mutex<TimedBufReader<R>>,
    write_stream: Mutex<TimedBufWriter<W>>,
}

impl<R: AsyncRead, W: AsyncWrite> Pipe<R,W>
{
    pub fn new(read_stream: R, write_stream: W) -> Pipe<R, W>
    {
        Self::from_buf(BufReader::new(read_stream), BufWriter::new(write_stream))
    }

    pub fn from_buf(read_stream: BufReader<R>, write_stream: BufWriter<W>) -> Pipe<R, W>
    {
        Pipe {
            read_stream: Mutex::new(TimedBufReader::from_buf(read_stream, DEFAULT_READ_TIMEOUT)),
            write_stream: Mutex::new(TimedBufWriter::from_buf(write_stream, DEFAULT_WRITE_TIMEOUT)),
        }
    }

    pub async fn get_read_timeout(&self) -> Duration
    {
        let stream = self.read_stream.lock().await;
        stream.get_timeout()
    }
    
    pub async fn set_read_timeout(&self, timeout: Duration)
    {
        let mut stream = self.read_stream.lock().await;
        stream.set_timeout(timeout)
    }

    pub async fn get_write_timeout(&self) -> Duration
    {
        let stream = self.write_stream.lock().await;
        stream.get_timeout()
    }

    pub async fn set_write_timeout(&self, timeout: Duration)
    {
        let mut stream = self.write_stream.lock().await;
        stream.set_timeout(timeout)
    }
}


