mod read;
mod traits;

pub use traits::*;

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

impl<R> CacheReader<R> {
    pub fn new(reader: R) -> CacheReader<R> {
        CacheReader {
            reader,
            cache: Vec::new(),
        }
    }
}
