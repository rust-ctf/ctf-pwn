//! Embassy runtime timer implementation.

use core::time::Duration;

use super::Timer;

/// Timer implementation backed by [`embassy_time`].
#[derive(Debug, Clone, Copy)]
pub struct EmbassyTimer;

impl Timer for EmbassyTimer {
    type Sleep = embassy_time::Timer;
    type Instant = embassy_time::Instant;

    fn sleep(duration: Duration) -> Self::Sleep {
        embassy_time::Timer::after(embassy_time::Duration::from_micros(
            #[expect(
                clippy::cast_possible_truncation,
                reason = "Duration over u64::MAX microseconds (~584,942 years) is unreachable in practice"
            )]
            {
                duration.as_micros() as u64
            },
        ))
    }

    fn sleep_until(deadline: Self::Instant) -> Self::Sleep {
        embassy_time::Timer::at(deadline)
    }

    fn now() -> Self::Instant {
        embassy_time::Instant::now()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;
    use crate::io::timer::{Timeout, test_helpers};

    fn mock_driver() -> &'static embassy_time::MockDriver {
        let driver = embassy_time::MockDriver::get();
        driver.reset();
        driver
    }

    /// Run an embassy task on a background thread with mock time advancement.
    fn run_with_mock_time(
        spawn_fn: fn(embassy_executor::Spawner),
        done: &'static AtomicBool,
        advance_ms: u64,
    ) {
        done.store(false, Ordering::SeqCst);
        mock_driver();

        std::thread::spawn(move || {
            let executor: &'static mut embassy_executor::Executor =
                Box::leak(Box::new(embassy_executor::Executor::new()));
            executor.run(|spawner| {
                spawn_fn(spawner);
            });
        });

        std::thread::sleep(Duration::from_millis(10));

        embassy_time::MockDriver::get()
            .advance(embassy_time::Duration::from_millis(advance_ms));

        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while !done.load(Ordering::SeqCst) {
            assert!(
                std::time::Instant::now() <= deadline,
                "embassy task did not complete within 2 seconds",
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }


    #[test]
    fn sleep_is_unpin() {
        test_helpers::sleep_is_unpin::<EmbassyTimer>();
    }

    #[test]
    fn timeout_from_duration() {
        test_helpers::timeout_from_duration::<EmbassyTimer>();
    }

    #[test]
    fn timeout_far_away_is_large() {
        test_helpers::timeout_far_away_is_large::<EmbassyTimer>();
    }


    #[test]
    fn now_returns_increasing_values() {
        let driver = mock_driver();
        let before = EmbassyTimer::now();
        driver.advance(embassy_time::Duration::from_millis(50));
        let after = EmbassyTimer::now();
        assert!(after > before);
    }

    #[test]
    fn now_stable_without_advance() {
        let _driver = mock_driver();
        let a = EmbassyTimer::now();
        let b = EmbassyTimer::now();
        assert!(b >= a);
    }


    static SLEEP_100_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_100_task() {
        EmbassyTimer::sleep(Duration::from_millis(100)).await;
        SLEEP_100_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_completes_after_advance() {
        run_with_mock_time(
            |s| { s.spawn(sleep_100_task()).expect("spawn"); },
            &SLEEP_100_DONE,
            200,
        );
    }

    static SLEEP_ZERO_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_zero_task() {
        EmbassyTimer::sleep(Duration::ZERO).await;
        SLEEP_ZERO_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_zero_completes() {
        run_with_mock_time(
            |s| { s.spawn(sleep_zero_task()).expect("spawn"); },
            &SLEEP_ZERO_DONE,
            1,
        );
    }


    static SLEEP_UNTIL_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_until_task() {
        let deadline = embassy_time::Instant::now() + embassy_time::Duration::from_millis(50);
        EmbassyTimer::sleep_until(deadline).await;
        SLEEP_UNTIL_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_until_future_deadline_completes() {
        run_with_mock_time(
            |s| { s.spawn(sleep_until_task()).expect("spawn"); },
            &SLEEP_UNTIL_DONE,
            100,
        );
    }


    static TIMEOUT_DELAY_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn timeout_delay_task() {
        let timeout: Timeout<EmbassyTimer> = Timeout::Delay(Duration::from_millis(50));
        timeout.into_sleep().await;
        TIMEOUT_DELAY_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timeout_delay_into_sleep_completes() {
        run_with_mock_time(
            |s| { s.spawn(timeout_delay_task()).expect("spawn"); },
            &TIMEOUT_DELAY_DONE,
            200,
        );
    }


    static SHORTER_FIRST_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn shorter_first_task() {
        use embassy_futures::select::{select, Either};

        let short = EmbassyTimer::sleep(Duration::from_millis(50));
        let long = EmbassyTimer::sleep(Duration::from_millis(500));

        match select(short, long).await {
            Either::First(()) => {}
            Either::Second(()) => unreachable!("long sleep completed first"),
        }
        SHORTER_FIRST_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn shorter_sleep_completes_first() {
        run_with_mock_time(
            |s| { s.spawn(shorter_first_task()).expect("spawn"); },
            &SHORTER_FIRST_DONE,
            100,
        );
    }

    static MULTI_WAKE_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn multi_wake_task() {
        use embassy_futures::select::{select, Either};

        let mut deadline = EmbassyTimer::sleep(Duration::from_millis(100));
        let mut count = 0u32;

        loop {
            let tick = EmbassyTimer::sleep(Duration::from_millis(20));
            match select(tick, &mut deadline).await {
                Either::First(()) => count += 1,
                Either::Second(()) => break,
            }
        }

        assert!(count >= 2, "expected at least 2 ticks before deadline, got {count}");
        MULTI_WAKE_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timer_survives_multiple_wakes_then_fires() {
        MULTI_WAKE_DONE.store(false, Ordering::SeqCst);
        mock_driver();

        std::thread::spawn(|| {
            let executor: &'static mut embassy_executor::Executor =
                Box::leak(Box::new(embassy_executor::Executor::new()));
            executor.run(|spawner| {
                spawner.spawn(multi_wake_task()).expect("spawn");
            });
        });

        // Advance in small increments — simulates data arriving in chunks
        for _ in 0..10 {
            std::thread::sleep(Duration::from_millis(5));
            embassy_time::MockDriver::get()
                .advance(embassy_time::Duration::from_millis(25));
        }

        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while !MULTI_WAKE_DONE.load(Ordering::SeqCst) {
            assert!(
                std::time::Instant::now() <= deadline,
                "embassy multi-wake task did not complete",
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}
