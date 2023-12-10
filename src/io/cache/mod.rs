mod read;
mod traits;

pub use read::*;
pub use traits::*;

use crate::io::timeout::HasTimeout;
use pin_project_lite::pin_project;
use std::time::Duration;

pin_project! {
    /// An `AsyncRead`er which applies a timeout to read operations.
    #[derive(Debug)]
    pub struct CacheReader<R> {
        #[pin]
        pub(crate) reader: R,
        #[pin]
        pub(crate) cache: Vec<u8>
    }
}

impl<R: HasTimeout> HasTimeout for CacheReader<R> {
    fn timeout(&self) -> Option<Duration> {
        self.reader.timeout()
    }

    fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.reader.set_timeout(timeout)
    }
}

impl<R> HasCache for CacheReader<R> {
    fn cache_clear(&mut self) {
        self.cache.clear()
    }

    fn cache_insert(&mut self, data: &[u8]) {
        self.cache.extend_from_slice(data)
    }

    fn has_cache(&self) -> bool {
        !self.cache.is_empty()
    }
}

impl<R> CacheReader<R> {
    pub fn new(reader: R) -> CacheReader<R> {
        CacheReader {
            reader,
            cache: Vec::new(),
        }
    }
}
