//! Cached reader that allows consuming and restoring buffered data.

mod read;
pub use read::*;

/// Trait for readers that support consuming and restoring cached bytes.
pub trait CacheRead {
    /// Consume `amt` bytes from the front of the cache.
    fn consume(&mut self, amt: usize);
    /// Restore `data` back into the cache for re-reading.
    fn restore(&mut self, data: &[u8]);
}
