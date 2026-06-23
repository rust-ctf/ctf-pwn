//! async-std runtime timer implementation.

use core::time::Duration;

use super::Timer;

/// Timer implementation using [`futures_timer::Delay`].
///
/// Works with async-std and any other runtime that supports standard
/// `Future` polling.
#[derive(Debug, Clone, Copy)]
pub struct AsyncStdTimer;

impl Timer for AsyncStdTimer {
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

#[cfg(test)]
mod tests {
    use core::future::Future;
    use core::pin::pin;
    use core::task::{Context, Waker};

    use super::*;
    use crate::io::timer::test_helpers;


    #[async_std::test]
    async fn sleep_completes() {
        test_helpers::sleep_completes::<AsyncStdTimer>().await;
    }

    #[async_std::test]
    async fn sleep_zero_completes() {
        test_helpers::sleep_zero_completes::<AsyncStdTimer>().await;
    }

    #[test]
    fn sleep_is_unpin() {
        test_helpers::sleep_is_unpin::<AsyncStdTimer>();
    }

    #[test]
    fn now_is_monotonic() {
        test_helpers::now_is_monotonic::<AsyncStdTimer>();
    }

    #[async_std::test]
    async fn timeout_delay_into_sleep_completes() {
        test_helpers::timeout_delay_into_sleep_completes::<AsyncStdTimer>().await;
    }

    #[test]
    fn timeout_from_duration() {
        test_helpers::timeout_from_duration::<AsyncStdTimer>();
    }

    #[test]
    fn timeout_far_away_is_large() {
        test_helpers::timeout_far_away_is_large::<AsyncStdTimer>();
    }


    #[async_std::test]
    async fn sleep_polls_pending_then_ready() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut fut = pin!(AsyncStdTimer::sleep(Duration::from_millis(30)));

        assert!(fut.as_mut().poll(&mut cx).is_pending());
        async_std::task::sleep(Duration::from_millis(50)).await;
        assert!(fut.as_mut().poll(&mut cx).is_ready());
    }

    #[async_std::test]
    async fn sleep_until_past_deadline_completes_immediately() {
        let past = AsyncStdTimer::now();
        async_std::task::sleep(Duration::from_millis(5)).await;
        AsyncStdTimer::sleep_until(past).await;
    }

    #[async_std::test]
    async fn sleep_until_future_deadline_completes() {
        let deadline = AsyncStdTimer::now() + Duration::from_millis(20);
        AsyncStdTimer::sleep_until(deadline).await;
        assert!(AsyncStdTimer::now() >= deadline);
    }

    #[async_std::test]
    async fn now_returns_ordered_after_sleep() {
        let before = AsyncStdTimer::now();
        async_std::task::sleep(Duration::from_millis(10)).await;
        let after = AsyncStdTimer::now();
        assert!(after > before);
    }

    #[async_std::test]
    async fn shorter_sleep_wakes_before_longer() {
        let start = AsyncStdTimer::now();

        let short = AsyncStdTimer::sleep(Duration::from_millis(20));
        let long = AsyncStdTimer::sleep(Duration::from_millis(200));

        futures::future::select(pin!(short), pin!(long)).await;

        let elapsed = AsyncStdTimer::now().duration_since(start);
        assert!(elapsed < Duration::from_millis(100));
    }

    #[async_std::test]
    async fn sleep_registers_waker_correctly() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut fut = pin!(AsyncStdTimer::sleep(Duration::from_millis(10)));

        let _ = fut.as_mut().poll(&mut cx);
        async_std::task::sleep(Duration::from_millis(20)).await;
        let result = fut.as_mut().poll(&mut cx);
        assert!(result.is_ready());
    }

    #[async_std::test]
    async fn timer_survives_multiple_wakes_then_fires() {
        let mut deadline = pin!(AsyncStdTimer::sleep(Duration::from_millis(100)));

        for _ in 0..5 {
            let tick = pin!(async_std::task::sleep(Duration::from_millis(10)));
            match futures::future::select(tick, &mut deadline).await {
                futures::future::Either::Left(_) => {}
                futures::future::Either::Right(_) => {
                    unreachable!("deadline should not fire during early iterations");
                }
            }
        }

        deadline.await;
    }

    #[async_std::test]
    async fn timer_fires_after_other_work_completes() {
        let mut timeout = pin!(AsyncStdTimer::sleep(Duration::from_millis(50)));
        let mut count = 0u32;

        loop {
            let tick = pin!(async_std::task::sleep(Duration::from_millis(10)));
            match futures::future::select(tick, &mut timeout).await {
                futures::future::Either::Left(_) => {
                    count += 1;
                }
                futures::future::Either::Right(_) => {
                    break;
                }
            }
        }

        assert!(count >= 3, "expected at least 3 iterations before timeout, got {count}");
    }
}
