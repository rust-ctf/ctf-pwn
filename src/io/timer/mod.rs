//! Runtime-agnostic timer abstraction.
//!
//! The [`Timer`] trait provides sleep/deadline operations that the selected
//! runtime implements. Exactly one `runtime-*` feature must be enabled.
//! [`Timeout`] wraps either a delay duration or an absolute deadline.

use core::future::Future;
use core::time::Duration;

#[cfg(feature = "runtime-tokio")]
pub mod tokio;

#[cfg(feature = "runtime-async-std")]
pub mod async_std;

#[cfg(feature = "runtime-smol")]
pub mod smol;

#[cfg(feature = "runtime-embassy")]
pub mod embassy;

#[cfg(test)]
#[cfg(not(feature = "runtime-embassy"))]
mod tests;

/// The timer implementation for the selected runtime.
#[cfg(feature = "runtime-tokio")]
pub type Timer = self::tokio::TokioTimer;

/// The timer implementation for the selected runtime.
#[cfg(feature = "runtime-async-std")]
pub type Timer = self::async_std::AsyncStdTimer;

/// The timer implementation for the selected runtime.
#[cfg(feature = "runtime-smol")]
pub type Timer = self::smol::SmolTimer;

/// The timer implementation for the selected runtime.
#[cfg(feature = "runtime-embassy")]
pub type Timer = self::embassy::EmbassyTimer;

/// Async timer abstraction.
///
/// Each runtime provides its own implementation with a concrete `Sleep`
/// future type and `Instant` timestamp type.
pub trait TimerProvider {
    /// A future that completes after a duration or at a deadline.
    type Sleep: Future<Output = ()> + Unpin;

    /// A point in time, as defined by this runtime.
    type Instant: Copy + Ord;

    /// Create a future that sleeps for the given duration.
    fn sleep(duration: Duration) -> Self::Sleep;

    /// Create a future that sleeps until the given deadline.
    fn sleep_until(deadline: Self::Instant) -> Self::Sleep;

    /// Return the current instant.
    fn now() -> Self::Instant;
}

/// The sleep future type for the selected runtime.
#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
pub type Sleep = <Timer as TimerProvider>::Sleep;

/// The instant type for the selected runtime.
#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
pub type Instant = <Timer as TimerProvider>::Instant;

/// Timeout specification as either a relative delay or an absolute deadline.
///
/// Uses the [`Timer`] selected by the runtime feature.
#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
#[derive(Debug, Clone, Copy)]
pub enum Timeout {
    /// A relative delay from now.
    Delay(Duration),
    /// An absolute point in time.
    Deadline(Instant),
}

#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
impl Timeout {
    /// Convert this timeout into a sleep future.
    #[must_use]
    pub fn into_sleep(self) -> Sleep {
        match self {
            Self::Delay(d) => Timer::sleep(d),
            Self::Deadline(i) => Timer::sleep_until(i),
        }
    }

    /// A very large duration (~30 years), used as "effectively no timeout".
    #[must_use]
    pub fn far_away() -> Duration {
        Duration::from_hours(262_800)
    }
}

#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
impl From<Duration> for Timeout {
    fn from(d: Duration) -> Self {
        Self::Delay(d)
    }
}

#[cfg(any(
    feature = "runtime-tokio",
    feature = "runtime-async-std",
    feature = "runtime-smol",
    feature = "runtime-embassy"
))]
impl From<Instant> for Timeout {
    fn from(i: Instant) -> Self {
        Self::Deadline(i)
    }
}

/// Shared test helpers for `Timer` implementations.
#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;

    #[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std", feature = "runtime-smol"))]
    pub(crate) async fn sleep_completes<T: TimerProvider>() {
        T::sleep(Duration::from_millis(10)).await;
    }

    #[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std", feature = "runtime-smol"))]
    pub(crate) async fn sleep_zero_completes<T: TimerProvider>() {
        T::sleep(Duration::ZERO).await;
    }

    pub(crate) fn sleep_is_unpin<T: TimerProvider>() {
        fn assert_unpin<U: Unpin>() {}
        assert_unpin::<T::Sleep>();
    }

    #[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std", feature = "runtime-smol"))]
    pub(crate) fn now_is_monotonic<T: TimerProvider>() {
        let mut prev = T::now();
        for _ in 0..10 {
            let curr = T::now();
            assert!(curr >= prev);
            prev = curr;
        }
    }

    #[cfg(any(feature = "runtime-tokio", feature = "runtime-async-std", feature = "runtime-smol"))]
    pub(crate) async fn timeout_delay_into_sleep_completes() {
        let timeout = Timeout::Delay(Duration::from_millis(10));
        timeout.into_sleep().await;
    }

    pub(crate) fn timeout_from_duration() {
        let timeout: Timeout = Duration::from_millis(42).into();
        assert!(matches!(timeout, Timeout::Delay(d) if d == Duration::from_millis(42)));
    }

    pub(crate) fn timeout_far_away_is_large() {
        let far = Timeout::far_away();
        assert!(far >= Duration::from_hours(262_800));
    }
}
