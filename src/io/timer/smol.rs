//! Smol runtime timer implementation.

use core::time::Duration;

use super::TimerProvider;

/// Timer implementation using [`futures_timer::Delay`].
///
/// Works with the smol async runtime and any other executor that drives
/// standard `Future` polling (e.g. `smol::block_on`).
#[derive(Debug, Clone, Copy)]
pub struct SmolTimer;

impl TimerProvider for SmolTimer {
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
}

