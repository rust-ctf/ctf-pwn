mod read;
pub use read::*;

pub trait CacheRead {
    fn consume(&mut self, amt: usize);
    fn restore(&mut self, data: &[u8]);
}
