mod convert;
mod io;

use std::time::Duration;
use tokio::{io::*, sync::Mutex};

pub use convert::*;
pub use io::*;

pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(1);
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(1);

pub struct Pipe<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> {
    read_stream: Mutex<TimedAsyncReader<R>>,
    write_stream: Mutex<TimedBufWriter<W>>,
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    pub fn new(read_stream: R, write_stream: W) -> Pipe<R, W> {
        Pipe {
            read_stream: Mutex::new(TimedAsyncReader::new(read_stream, DEFAULT_READ_TIMEOUT)),
            write_stream: Mutex::new(TimedBufWriter::new(write_stream, DEFAULT_WRITE_TIMEOUT)),
        }
    }

    pub async fn get_read_timeout(&self) -> Duration {
        let stream = self.read_stream.lock().await;
        stream.get_timeout()
    }

    pub async fn set_read_timeout(&self, timeout: Duration) {
        let mut stream = self.read_stream.lock().await;
        stream.set_timeout(timeout)
    }

    pub async fn get_write_timeout(&self) -> Duration {
        let stream = self.write_stream.lock().await;
        stream.get_timeout()
    }

    pub async fn set_write_timeout(&self, timeout: Duration) {
        let mut stream = self.write_stream.lock().await;
        stream.set_timeout(timeout)
    }
}
