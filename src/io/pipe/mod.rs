mod error;
mod read;
mod write;

pub use error::*;
pub use read::*;

use pin_project_lite::pin_project;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

use super::{cache::*, timeout::*};

pin_project! {
    /// An `AsyncRead`er which applies a timeout to read operations.
    #[derive(Debug)]
    pub struct Pipe<R,W> {
        #[pin]
        reader: TimeoutReader<R>,
        #[pin]
        writer: TimeoutWriter<W>,
    }
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

    pub fn new(reader: R, writer: W) -> Pipe<R, W> {
        let mut timeout_reader = TimeoutReader::new(reader);
        let mut timeout_writer = TimeoutWriter::new(writer);

        timeout_reader.set_timeout(Some(Self::DEFAULT_TIMEOUT));
        timeout_writer.set_timeout(Some(Self::DEFAULT_TIMEOUT));

        Pipe {
            reader: timeout_reader, //CacheReader::new(timeout_reader),
            writer: timeout_writer,
        }
    }
}
