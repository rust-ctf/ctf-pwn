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

#[cfg(test)]
mod tests {
    use core::future::Future;
    use core::pin::pin;
    use core::task::{Context, Waker};

    use super::*;
    use crate::io::timer::{Timeout, test_helpers};

    #[tokio::test]
    async fn sleep_completes() {
        test_helpers::sleep_completes::<TokioTimer>().await;
    }

    #[tokio::test]
    async fn sleep_zero_completes() {
        test_helpers::sleep_zero_completes::<TokioTimer>().await;
    }

    #[test]
    fn sleep_is_unpin() {
        test_helpers::sleep_is_unpin::<TokioTimer>();
    }

    #[test]
    fn now_is_monotonic() {
        test_helpers::now_is_monotonic::<TokioTimer>();
    }

    #[tokio::test]
    async fn timeout_delay_into_sleep_completes() {
        test_helpers::timeout_delay_into_sleep_completes().await;
    }

    #[test]
    fn timeout_from_duration() {
        test_helpers::timeout_from_duration();
    }

    #[test]
    fn timeout_far_away_is_large() {
        test_helpers::timeout_far_away_is_large();
    }

    #[tokio::test]
    async fn sleep_polls_pending_then_ready() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut fut = pin!(TokioTimer::sleep(Duration::from_millis(50)));

        assert!(fut.as_mut().poll(&mut cx).is_pending());
        ::tokio::time::sleep(Duration::from_millis(60)).await;
        assert!(fut.as_mut().poll(&mut cx).is_ready());
    }

    #[tokio::test]
    async fn sleep_until_past_deadline_completes_immediately() {
        let past = TokioTimer::now();
        ::tokio::time::sleep(Duration::from_millis(5)).await;
        TokioTimer::sleep_until(past).await;
    }

    #[tokio::test]
    async fn sleep_until_future_deadline_completes() {
        let deadline = TokioTimer::now() + Duration::from_millis(20);
        TokioTimer::sleep_until(deadline).await;
        assert!(TokioTimer::now() >= deadline);
    }

    #[tokio::test]
    async fn sleep_registers_waker_correctly() {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut fut: core::pin::Pin<Box<_>> = TokioTimer::sleep(Duration::from_millis(10));

        let _ = core::pin::Pin::new(&mut fut).poll(&mut cx);

        ::tokio::time::sleep(Duration::from_millis(20)).await;
        let result = core::pin::Pin::new(&mut fut).poll(&mut cx);
        assert!(result.is_ready());
    }

    #[tokio::test]
    async fn now_returns_ordered_after_sleep() {
        let before = TokioTimer::now();
        ::tokio::time::sleep(Duration::from_millis(10)).await;
        let after = TokioTimer::now();
        assert!(after > before);
    }

    #[tokio::test]
    async fn timeout_deadline_into_sleep_completes() {
        let deadline = TokioTimer::now() + Duration::from_millis(20);
        let timeout = Timeout::Deadline(deadline);
        timeout.into_sleep().await;
        assert!(TokioTimer::now() >= deadline);
    }

    #[tokio::test]
    async fn timeout_from_instant() {
        let instant = TokioTimer::now() + Duration::from_millis(10);
        let timeout = instant.into();
        assert!(matches!(timeout, Timeout::Deadline(_)));
        timeout.into_sleep().await;
    }

    #[tokio::test]
    async fn timeout_copy_produces_independent_sleeps() {
        let timeout = Timeout::Delay(Duration::from_millis(10));
        let timeout2 = timeout;
        let ((), ()) = ::tokio::join!(timeout.into_sleep(), timeout2.into_sleep());
    }

    #[tokio::test]
    async fn shorter_sleep_wakes_before_longer() {
        let start = TokioTimer::now();

        let short = TokioTimer::sleep(Duration::from_millis(20));
        let long = TokioTimer::sleep(Duration::from_millis(200));

        ::tokio::select! {
            () = short => {}
            () = long => { unreachable!("long sleep should not complete first"); }
        }

        let elapsed = TokioTimer::now().duration_since(start);
        assert!(elapsed < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn timer_survives_multiple_wakes_then_fires() {
        let mut deadline = pin!(TokioTimer::sleep(Duration::from_millis(100)));

        for _ in 0..5 {
            ::tokio::select! {
                biased;
                () = ::tokio::time::sleep(Duration::from_millis(10)) => {}
                () = &mut deadline => {
                    unreachable!("deadline should not fire during early iterations");
                }
            }
        }

        // Now wait for the deadline to actually fire
        deadline.await;
    }

    #[tokio::test]
    async fn timer_fires_after_other_work_completes() {
        let mut timeout = pin!(TokioTimer::sleep(Duration::from_millis(50)));
        let mut count = 0u32;

        loop {
            ::tokio::select! {
                biased;
                () = ::tokio::time::sleep(Duration::from_millis(10)) => {
                    count += 1;
                }
                () = &mut timeout => {
                    break;
                }
            }
        }

        assert!(count >= 3, "expected at least 3 iterations before timeout, got {count}");
    }
}
