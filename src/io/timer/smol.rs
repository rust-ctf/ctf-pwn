//! Smol runtime timer implementation.

use core::time::Duration;

use super::Timer;

/// Timer implementation using [`futures_timer::Delay`].
///
/// Works with the smol async runtime and any other executor that drives
/// standard `Future` polling (e.g. `smol::block_on`).
#[derive(Debug, Clone, Copy)]
pub struct SmolTimer;

impl Timer for SmolTimer {
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
    use crate::io::timer::Timeout;

    #[test]
    fn sleep_completes() {
        smol::block_on(test_helpers::sleep_completes::<SmolTimer>());
    }

    #[test]
    fn sleep_zero_completes() {
        smol::block_on(test_helpers::sleep_zero_completes::<SmolTimer>());
    }

    #[test]
    fn sleep_is_unpin() {
        test_helpers::sleep_is_unpin::<SmolTimer>();
    }

    #[test]
    fn now_is_monotonic() {
        test_helpers::now_is_monotonic::<SmolTimer>();
    }

    #[test]
    fn timeout_delay_into_sleep_completes() {
        smol::block_on(test_helpers::timeout_delay_into_sleep_completes::<SmolTimer>());
    }

    #[test]
    fn timeout_from_duration() {
        test_helpers::timeout_from_duration::<SmolTimer>();
    }

    #[test]
    fn timeout_far_away_is_large() {
        test_helpers::timeout_far_away_is_large::<SmolTimer>();
    }

    #[test]
    fn sleep_polls_pending_then_ready() {
        smol::block_on(async {
            let waker = Waker::noop();
            let mut cx = Context::from_waker(waker);
            let mut fut = pin!(SmolTimer::sleep(Duration::from_millis(30)));

            assert!(fut.as_mut().poll(&mut cx).is_pending());
            smol::Timer::after(Duration::from_millis(50)).await;
            assert!(fut.as_mut().poll(&mut cx).is_ready());
        });
    }

    #[test]
    fn sleep_until_past_deadline_completes_immediately() {
        smol::block_on(async {
            let past = SmolTimer::now();
            smol::Timer::after(Duration::from_millis(5)).await;
            SmolTimer::sleep_until(past).await;
        });
    }

    #[test]
    fn sleep_until_future_deadline_completes() {
        smol::block_on(async {
            let deadline = SmolTimer::now() + Duration::from_millis(20);
            SmolTimer::sleep_until(deadline).await;
            assert!(SmolTimer::now() >= deadline);
        });
    }

    #[test]
    fn now_returns_ordered_after_sleep() {
        smol::block_on(async {
            let before = SmolTimer::now();
            smol::Timer::after(Duration::from_millis(10)).await;
            let after = SmolTimer::now();
            assert!(after > before);
        });
    }

    #[test]
    fn shorter_sleep_wakes_before_longer() {
        smol::block_on(async {
            let start = SmolTimer::now();

            let short = SmolTimer::sleep(Duration::from_millis(20));
            let long = SmolTimer::sleep(Duration::from_millis(200));

            futures::future::select(pin!(short), pin!(long)).await;

            let elapsed = SmolTimer::now().duration_since(start);
            assert!(elapsed < Duration::from_millis(100));
        });
    }

    #[test]
    fn timer_survives_multiple_wakes_then_fires() {
        smol::block_on(async {
            let mut deadline = pin!(SmolTimer::sleep(Duration::from_millis(100)));

            for _ in 0..5 {
                let tick = pin!(smol::Timer::after(Duration::from_millis(10)));
                match futures::future::select(tick, &mut deadline).await {
                    futures::future::Either::Left(_) => {}
                    futures::future::Either::Right(_) => {
                        unreachable!("deadline should not fire during early iterations");
                    }
                }
            }

            deadline.await;
        });
    }

    #[test]
    fn timer_fires_after_other_work_completes() {
        smol::block_on(async {
            let mut timeout = pin!(SmolTimer::sleep(Duration::from_millis(50)));
            let mut count = 0u32;

            loop {
                let tick = pin!(smol::Timer::after(Duration::from_millis(10)));
                match futures::future::select(tick, &mut timeout).await {
                    futures::future::Either::Left(_) => count += 1,
                    futures::future::Either::Right(_) => break,
                }
            }

            assert!(
                count >= 3,
                "expected at least 3 iterations before timeout, got {count}"
            );
        });
    }

    #[test]
    fn timeout_from_instant() {
        smol::block_on(async {
            let deadline = SmolTimer::now() + Duration::from_millis(10);
            let timeout: Timeout<SmolTimer> = Timeout::Deadline(deadline);
            timeout.into_sleep().await;
            assert!(SmolTimer::now() >= deadline);
        });
    }
}
