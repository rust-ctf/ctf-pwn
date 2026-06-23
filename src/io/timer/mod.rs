//! Runtime-agnostic timer abstraction.
//!
//! The [`Timer`] trait provides sleep/deadline operations that each runtime
//! implements. [`Timeout`] wraps either a delay duration or an absolute
//! deadline.

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

/// Async timer abstraction.
///
/// Each runtime provides its own implementation with a concrete `Sleep`
/// future type and `Instant` timestamp type.
pub trait Timer {
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

/// Timeout specification as either a relative delay or an absolute deadline.
#[derive(Debug, Clone, Copy)]
pub enum Timeout<T: Timer> {
    /// A relative delay from now.
    Delay(Duration),
    /// An absolute point in time.
    Deadline(T::Instant),
}

impl<T: Timer> Timeout<T> {
    /// Convert this timeout into a sleep future.
    #[must_use]
    pub fn into_sleep(self) -> T::Sleep {
        match self {
            Self::Delay(d) => T::sleep(d),
            Self::Deadline(i) => T::sleep_until(i),
        }
    }

    /// A very large duration (~30 years), used as "effectively no timeout".
    #[must_use]
    pub fn far_away() -> Duration {
        Duration::from_hours(262_800)
    }
}

impl<T: Timer> From<Duration> for Timeout<T> {
    fn from(d: Duration) -> Self {
        Self::Delay(d)
    }
}

/// Shared test helpers for all `Timer` implementations.
///
/// Each runtime module calls these generic async fns from its own
/// `#[tokio::test]` / `#[async_std::test]` / executor wrappers.
#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;

    pub(crate) async fn sleep_completes<T: Timer>() {
        T::sleep(Duration::from_millis(10)).await;
    }

    pub(crate) async fn sleep_zero_completes<T: Timer>() {
        T::sleep(Duration::ZERO).await;
    }

    pub(crate) fn sleep_is_unpin<T: Timer>() {
        fn assert_unpin<U: Unpin>() {}
        assert_unpin::<T::Sleep>();
    }

    pub(crate) fn now_is_monotonic<T: Timer>() {
        let mut prev = T::now();
        for _ in 0..10 {
            let curr = T::now();
            assert!(curr >= prev);
            prev = curr;
        }
    }

    pub(crate) async fn timeout_delay_into_sleep_completes<T: Timer>() {
        let timeout: Timeout<T> = Timeout::Delay(Duration::from_millis(10));
        timeout.into_sleep().await;
    }

    pub(crate) fn timeout_from_duration<T: Timer>() {
        let timeout: Timeout<T> = Duration::from_millis(42).into();
        assert!(matches!(timeout, Timeout::Delay(d) if d == Duration::from_millis(42)));
    }

    pub(crate) fn timeout_far_away_is_large<T: Timer>() {
        let far = Timeout::<T>::far_away();
        assert!(far >= Duration::from_hours(262_800));
    }
}
