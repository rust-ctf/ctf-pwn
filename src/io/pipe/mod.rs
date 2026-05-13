//! Pipe abstractions for bidirectional async I/O.

mod error;
mod protocol;
mod read;
mod wrapper;
pub use error::*;
pub use protocol::*;
pub use read::*;
pub use wrapper::*;

use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite};

use super::cache::CacheRead;

/// Async readable pipe with timeout and caching support.
pub trait PipeRead: AsyncRead + CacheRead {
    /// Default timeout for read operations.
    const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);
    /// Returns the current read timeout, if set.
    fn read_timeout(&self) -> Option<Duration>;
    /// Sets the read timeout.
    fn set_read_timeout(&mut self, timeout: Option<Duration>);
}
/// Async writable pipe with timeout support.
pub trait PipeWrite: AsyncWrite {
    /// Default timeout for write operations.
    const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(5);

    /// Returns the current write timeout, if set.
    fn write_timeout(&self) -> Option<Duration>;
    /// Sets the write timeout.
    fn set_write_timeout(&mut self, timeout: Option<Duration>);
}

/// Combined readable and writable pipe.
pub trait Pipe: PipeRead + PipeWrite {
    /// Split into separate read and write halves.
    fn split(&mut self) -> (&mut impl PipeRead, &mut impl PipeWrite);
    /// Returns mutable reference to the reader half.
    fn reader(&mut self) -> &mut impl PipeRead;
    /// Returns mutable reference to the writer half.
    fn writer(&mut self) -> &mut impl PipeWrite;
}
