//! Extension traits and futures for async reads with timeouts.

pub(crate) mod read_buf_timeout;
pub(crate) mod read_exact_timeout;
pub(crate) mod read_int_timeout;
pub(crate) mod read_timeout;
mod read_timeout_ext;
pub use read_timeout_ext::*;

/// Polls a read and falls back to timeout on `Pending`.
#[macro_export]
macro_rules! timeout_ready {
    ($e:expr $(,)?, $d:expr $(,)?, $c:expr $(,)?, $cx:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => {
                let delay = $d.get_or_insert_with(|| Box::pin($c.timeout()));

                return match delay.as_mut().poll($cx) {
                    Poll::Pending => std::task::Poll::Pending,
                    Poll::Ready(()) => std::task::Poll::Ready(Err(IOTimeoutError::Timeout)),
                };
            }
        }
    };
}
