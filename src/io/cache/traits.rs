pub trait HasCache {
    fn cache_clear(&mut self);
    fn cache_insert(&mut self, data: &[u8]);
    fn has_cache(&self) -> bool;
}

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
pub use tokio::io::AsyncBufRead;
use tokio::io::{AsyncRead, ReadBuf};


pub trait AsyncCacheRead : AsyncRead
{
    fn poll_reader(self: Pin<&mut Self>, cx: &mut Context<'_>,buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>>;

    fn consume(self: Pin<&mut Self>, amt: usize);
    fn restore(self: Pin<&mut Self>, data: &[u8]);
}