//! Embassy runtime timer implementation.

use core::time::Duration;

use super::TimerProvider;

/// Convert [`core::time::Duration`] to [`embassy_time::Duration`].
///
/// Saturates to the maximum representable embassy duration (~584,942 years)
/// if the input exceeds `u64::MAX` microseconds.
fn to_embassy_duration(duration: Duration) -> embassy_time::Duration {
    embassy_time::Duration::try_from(duration).unwrap_or(embassy_time::Duration::MAX)
}

/// Timer implementation backed by [`embassy_time`].
#[derive(Debug, Clone, Copy)]
pub struct EmbassyTimer;

impl TimerProvider for EmbassyTimer {
    type Sleep = embassy_time::Timer;
    type Instant = embassy_time::Instant;

    fn sleep(duration: Duration) -> Self::Sleep {
        embassy_time::Timer::after(to_embassy_duration(duration))
    }

    fn sleep_until(deadline: Self::Instant) -> Self::Sleep {
        embassy_time::Timer::at(deadline)
    }

    fn now() -> Self::Instant {
        embassy_time::Instant::now()
    }

    fn deadline(duration: Duration) -> Self::Instant {
        embassy_time::Instant::now() + to_embassy_duration(duration)
    }
}
