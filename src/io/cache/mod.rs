mod read;
pub use read::*;

use tokio::io::AsyncRead;

pub trait CacheRead: AsyncRead {
    fn consume(&mut self, amt: usize);
    fn restore(&mut self, data: &[u8]);
}
