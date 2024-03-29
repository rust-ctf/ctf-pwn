mod async_read_cache_timeout_ext;
mod async_read_timeout_ext;
mod read_exact_timeout;
mod read_timeout;
mod read_to_end_timeout;
mod read_until_regex_timeout;
mod read_until_timeout;

pub use async_read_cache_timeout_ext::*;
pub use async_read_timeout_ext::*;
pub use read_exact_timeout::*;
pub use read_timeout::*;
pub use read_until_regex_timeout::*;
pub use read_until_timeout::*;
use std::io;
use std::io::ErrorKind;

use std::time::Duration;
use tokio::time::Instant;

pub(crate) fn get_deadline(timeout: Duration) -> Instant {
    Instant::now()
        .checked_add(timeout)
        .unwrap_or_else(|| far_future())
}

pub(crate) fn far_future() -> Instant {
    // Roughly 30 years from now.
    // API does not provide a way to obtain max `Instant`
    // or convert specific date in the future to instant.
    // 1000 years overflows on macOS, 100 years overflows on FreeBSD.
    Instant::now() + Duration::from_secs(86400 * 365 * 30)
}

fn eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "EOF")
}
fn timeout() -> io::Error {
    io::Error::new(ErrorKind::TimedOut, "Timeout")
}
