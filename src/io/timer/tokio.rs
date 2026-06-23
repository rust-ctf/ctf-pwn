//! Tokio runtime timer implementation.

use core::pin::Pin;
use core::time::Duration;

use super::TimerProvider;

/// Timer implementation backed by [`tokio::time`].
#[derive(Debug, Clone, Copy)]
pub struct TokioTimer;

impl TimerProvider for TokioTimer {
    type Sleep = Pin<Box<::tokio::time::Sleep>>;
    type Instant = ::tokio::time::Instant;

    fn sleep(duration: Duration) -> Self::Sleep {
        Box::pin(::tokio::time::sleep(duration))
    }

    fn sleep_until(deadline: Self::Instant) -> Self::Sleep {
        Box::pin(::tokio::time::sleep_until(deadline))
    }

    fn now() -> Self::Instant {
        ::tokio::time::Instant::now()
    }
}

