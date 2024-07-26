mod error;
mod read;
mod wrapper;
pub use error::*;
pub use read::*;
pub use wrapper::*;

use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite};

use super::cache::CacheRead;

pub trait PipeRead: AsyncRead + CacheRead {
    const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);
    fn read_timeout(&self) -> Option<Duration>;
    fn set_read_timeout(&mut self, timeout: Option<Duration>);
}
pub trait PipeWrite: AsyncWrite {
    const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(5);

    fn write_timeout(&self) -> Option<Duration>;
    fn set_write_timeout(&mut self, timeout: Option<Duration>);
}

pub trait Pipe: PipeRead + PipeWrite {
    fn split(&mut self) -> (&mut impl PipeRead, &mut impl PipeWrite);
    fn reader(&mut self) -> &mut impl PipeRead;
    fn writer(&mut self) -> &mut impl PipeWrite;
}
