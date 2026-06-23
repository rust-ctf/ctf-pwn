//! async-std runtime timer implementation.

use core::time::Duration;

use super::TimerProvider;

/// Timer implementation using [`futures_timer::Delay`].
///
/// Works with async-std and any other runtime that supports standard
/// `Future` polling.
#[derive(Debug, Clone, Copy)]
pub struct AsyncStdTimer;

impl TimerProvider for AsyncStdTimer {
    type Sleep = futures_timer::Delay;
    type Instant = std::time::Instant;

    fn sleep(duration: Duration) -> Self::Sleep {
        futures_timer::Delay::new(duration)
    }

    fn sleep_until(deadline: Self::Instant) -> Self::Sleep {
        let now = std::time::Instant::now();
        let duration = deadline.saturating_duration_since(now);
        futures_timer::Delay::new(duration)
    }

    fn now() -> Self::Instant {
        std::time::Instant::now()
    }

    fn deadline(duration: Duration) -> Self::Instant {
        Self::now() + duration
    }
}

