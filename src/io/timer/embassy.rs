//! Embassy runtime timer implementation.

use core::time::Duration;

use super::TimerProvider;

/// Timer implementation backed by [`embassy_time`].
#[derive(Debug, Clone, Copy)]
pub struct EmbassyTimer;

impl TimerProvider for EmbassyTimer {
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

    /// Run an embassy task on a background thread with real time.
    /// The task must set `done` to true when finished.
    fn run_embassy(
        spawn_fn: fn(embassy_executor::Spawner),
        done: &'static AtomicBool,
    ) {
        done.store(false, Ordering::SeqCst);

        std::thread::spawn(move || {
            let executor: &'static mut embassy_executor::Executor =
                Box::leak(Box::new(embassy_executor::Executor::new()));
            executor.run(|spawner| {
                spawn_fn(spawner);
            });
        });

        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while !done.load(Ordering::SeqCst) {
            assert!(
                std::time::Instant::now() <= deadline,
                "embassy task did not complete within 5 seconds",
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn sleep_is_unpin() {
        test_helpers::sleep_is_unpin::<EmbassyTimer>();
    }

    static SLEEP_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_task() {
        EmbassyTimer::sleep(Duration::from_millis(50)).await;
        SLEEP_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_completes() {
        let start = std::time::Instant::now();
        run_embassy(
            |s| { s.spawn(sleep_task()).expect("spawn"); },
            &SLEEP_DONE,
        );
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(40), "sleep too fast: {elapsed:?}");
        assert!(elapsed < Duration::from_millis(500), "sleep too slow: {elapsed:?}");
    }

    static SLEEP_ZERO_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_zero_task() {
        EmbassyTimer::sleep(Duration::ZERO).await;
        SLEEP_ZERO_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_zero_completes() {
        run_embassy(
            |s| { s.spawn(sleep_zero_task()).expect("spawn"); },
            &SLEEP_ZERO_DONE,
        );
    }

    static SLEEP_UNTIL_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_until_task() {
        let deadline = embassy_time::Instant::now() + embassy_time::Duration::from_millis(50);
        EmbassyTimer::sleep_until(deadline).await;
        assert!(embassy_time::Instant::now() >= deadline);
        SLEEP_UNTIL_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_until_future_deadline_completes() {
        run_embassy(
            |s| { s.spawn(sleep_until_task()).expect("spawn"); },
            &SLEEP_UNTIL_DONE,
        );
    }

    static SLEEP_UNTIL_PAST_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_until_past_task() {
        let past = embassy_time::Instant::from_ticks(0);
        EmbassyTimer::sleep_until(past).await;
        SLEEP_UNTIL_PAST_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_until_past_deadline_completes() {
        run_embassy(
            |s| { s.spawn(sleep_until_past_task()).expect("spawn"); },
            &SLEEP_UNTIL_PAST_DONE,
        );
    }

    static POLLS_PENDING_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn polls_pending_task() {
        use core::future::Future;
        use core::pin::pin;

        let mut fut = pin!(EmbassyTimer::sleep(Duration::from_millis(100)));

        let is_pending = core::future::poll_fn(|cx| {
            let result = fut.as_mut().poll(cx);
            core::task::Poll::Ready(result.is_pending())
        }).await;
        assert!(is_pending, "first poll should be Pending");

        fut.await;
        POLLS_PENDING_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn sleep_polls_pending_then_ready() {
        run_embassy(
            |s| { s.spawn(polls_pending_task()).expect("spawn"); },
            &POLLS_PENDING_DONE,
        );
    }

    static NOW_ORDERED_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn now_ordered_task() {
        let before = EmbassyTimer::now();
        EmbassyTimer::sleep(Duration::from_millis(10)).await;
        let after = EmbassyTimer::now();
        assert!(after > before);
        NOW_ORDERED_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn now_returns_ordered_after_sleep() {
        run_embassy(
            |s| { s.spawn(now_ordered_task()).expect("spawn"); },
            &NOW_ORDERED_DONE,
        );
    }

    #[test]
    fn now_is_monotonic() {
        let mut prev = EmbassyTimer::now();
        for _ in 0..10 {
            let curr = EmbassyTimer::now();
            assert!(curr >= prev);
            prev = curr;
        }
    }

    static TIMEOUT_DELAY_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn timeout_delay_task() {
        let timeout = Timeout::Delay(Duration::from_millis(50));
        timeout.into_sleep().await;
        TIMEOUT_DELAY_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timeout_delay_into_sleep_completes() {
        run_embassy(
            |s| { s.spawn(timeout_delay_task()).expect("spawn"); },
            &TIMEOUT_DELAY_DONE,
        );
    }

    static TIMEOUT_DEADLINE_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn timeout_deadline_task() {
        let deadline = embassy_time::Instant::now() + embassy_time::Duration::from_millis(50);
        let timeout = Timeout::Deadline(deadline);
        timeout.into_sleep().await;
        TIMEOUT_DEADLINE_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timeout_deadline_into_sleep_completes() {
        run_embassy(
            |s| { s.spawn(timeout_deadline_task()).expect("spawn"); },
            &TIMEOUT_DEADLINE_DONE,
        );
    }

    #[test]
    fn timeout_from_duration() {
        test_helpers::timeout_from_duration();
    }

    #[test]
    fn timeout_from_instant() {
        let instant = EmbassyTimer::now();
        let timeout: Timeout = instant.into();
        assert!(matches!(timeout, Timeout::Deadline(_)));
    }

    #[test]
    fn timeout_far_away_is_large() {
        test_helpers::timeout_far_away_is_large();
    }

    static TIMEOUT_COPY_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn timeout_copy_task() {
        let timeout = Timeout::Delay(Duration::from_millis(10));
        let timeout2 = timeout;
        embassy_futures::join::join(timeout.into_sleep(), timeout2.into_sleep()).await;
        TIMEOUT_COPY_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timeout_copy_produces_independent_sleeps() {
        run_embassy(
            |s| { s.spawn(timeout_copy_task()).expect("spawn"); },
            &TIMEOUT_COPY_DONE,
        );
    }

    static SHORTER_FIRST_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn shorter_first_task() {
        use embassy_futures::select::{select, Either};

        let short = EmbassyTimer::sleep(Duration::from_millis(20));
        let long = EmbassyTimer::sleep(Duration::from_millis(500));

        match select(short, long).await {
            Either::First(()) => {}
            Either::Second(()) => unreachable!("long sleep completed first"),
        }
        SHORTER_FIRST_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn shorter_sleep_completes_first() {
        let start = std::time::Instant::now();
        run_embassy(
            |s| { s.spawn(shorter_first_task()).expect("spawn"); },
            &SHORTER_FIRST_DONE,
        );
        assert!(start.elapsed() < Duration::from_millis(200));
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

        assert!(count >= 2, "expected at least 2 ticks, got {count}");
        MULTI_WAKE_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timer_survives_multiple_wakes_then_fires() {
        run_embassy(
            |s| { s.spawn(multi_wake_task()).expect("spawn"); },
            &MULTI_WAKE_DONE,
        );
    }

    static FIRES_AFTER_WORK_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn fires_after_work_task() {
        use embassy_futures::select::{select, Either};

        let mut timeout = EmbassyTimer::sleep(Duration::from_millis(100));
        let mut count = 0u32;

        loop {
            let tick = EmbassyTimer::sleep(Duration::from_millis(20));
            match select(tick, &mut timeout).await {
                Either::First(()) => count += 1,
                Either::Second(()) => break,
            }
        }

        assert!(count >= 2, "expected at least 2 ticks, got {count}");
        FIRES_AFTER_WORK_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn timer_fires_after_other_work_completes() {
        run_embassy(
            |s| { s.spawn(fires_after_work_task()).expect("spawn"); },
            &FIRES_AFTER_WORK_DONE,
        );
    }

    static MOCK_READER_SLEEP_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn mock_reader_sleep_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(50)),
            ReadAction::Data(b"after_sleep".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let start = embassy_time::Instant::now();
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await
            .unwrap_or(0);
        let elapsed_ms = (embassy_time::Instant::now() - start).as_millis();

        assert_eq!(&buf[..n], b"after_sleep");
        assert!(elapsed_ms >= 40, "expected ~50ms, got {elapsed_ms}ms");
        MOCK_READER_SLEEP_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_sleep_then_data() {
        run_embassy(
            |s| { s.spawn(mock_reader_sleep_task()).expect("spawn"); },
            &MOCK_READER_SLEEP_DONE,
        );
    }

    static MOCK_READER_CHUNKS_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn mock_reader_chunks_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[
            ReadAction::Data(b"hello".to_vec()),
            ReadAction::Sleep(Duration::from_millis(20)),
            ReadAction::Data(b"world".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"hello");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"world");
        MOCK_READER_CHUNKS_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_data_with_sleeps() {
        run_embassy(
            |s| { s.spawn(mock_reader_chunks_task()).expect("spawn"); },
            &MOCK_READER_CHUNKS_DONE,
        );
    }

    static MOCK_READER_PARTIAL_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn mock_reader_partial_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[ReadAction::Data(b"abcdef".to_vec())]);
        let mut buf = [0u8; 3];

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"abc");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"def");
        MOCK_READER_PARTIAL_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_partial_when_buf_small() {
        run_embassy(
            |s| { s.spawn(mock_reader_partial_task()).expect("spawn"); },
            &MOCK_READER_PARTIAL_DONE,
        );
    }

    static MOCK_READER_MULTI_SLEEP_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn mock_reader_multi_sleep_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[
            ReadAction::Data(b"a".to_vec()),
            ReadAction::Sleep(Duration::from_millis(20)),
            ReadAction::Data(b"b".to_vec()),
            ReadAction::Sleep(Duration::from_millis(20)),
            ReadAction::Data(b"c".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let start = embassy_time::Instant::now();
        for expected in [b"a".as_slice(), b"b", b"c"] {
            let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
                .await.unwrap_or(0);
            assert_eq!(&buf[..n], expected);
        }
        let elapsed_ms = (embassy_time::Instant::now() - start).as_millis();
        assert!(elapsed_ms >= 30, "two 20ms sleeps should take >=30ms, got {elapsed_ms}ms");
        MOCK_READER_MULTI_SLEEP_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_multiple_sleeps_accumulate() {
        run_embassy(
            |s| { s.spawn(mock_reader_multi_sleep_task()).expect("spawn"); },
            &MOCK_READER_MULTI_SLEEP_DONE,
        );
    }

    static MOCK_READER_NO_SLEEP_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn mock_reader_no_sleep_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[ReadAction::Data(b"fast".to_vec())]);
        let mut buf = [0u8; 64];

        let start = embassy_time::Instant::now();
        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        let elapsed_ms = (embassy_time::Instant::now() - start).as_millis();

        assert_eq!(&buf[..n], b"fast");
        assert!(elapsed_ms < 10, "data-only should be instant, got {elapsed_ms}ms");
        MOCK_READER_NO_SLEEP_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_no_sleep_is_instant() {
        run_embassy(
            |s| { s.spawn(mock_reader_no_sleep_task()).expect("spawn"); },
            &MOCK_READER_NO_SLEEP_DONE,
        );
    }

    static DATA_CHUNKS_EOF_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn data_chunks_eof_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[
            ReadAction::Data(b"hello".to_vec()),
            ReadAction::Data(b"world".to_vec()),
        ]);
        let mut buf = [0u8; 64];

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"hello");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(&buf[..n], b"world");

        let n = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf))
            .await.unwrap_or(0);
        assert_eq!(n, 0);
        DATA_CHUNKS_EOF_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_data_chunks_and_eof() {
        run_embassy(
            |s| { s.spawn(data_chunks_eof_task()).expect("spawn"); },
            &DATA_CHUNKS_EOF_DONE,
        );
    }

    static SLEEP_ACCURATE_DONE: AtomicBool = AtomicBool::new(false);

    #[embassy_executor::task]
    async fn sleep_accurate_task() {
        use core::pin::Pin;
        use crate::io::read::Read;
        use crate::io::test_utils::{MockReader, ReadAction};

        let mut reader = MockReader::new(&[
            ReadAction::Sleep(Duration::from_millis(100)),
            ReadAction::Data(b"x".to_vec()),
        ]);
        let mut buf = [0u8; 1];

        let start = embassy_time::Instant::now();
        let _ = core::future::poll_fn(|cx| Pin::new(&mut reader).poll_read(cx, &mut buf)).await;
        let elapsed_ms = (embassy_time::Instant::now() - start).as_millis();

        assert!(elapsed_ms >= 80, "expected ~100ms, got {elapsed_ms}ms");
        assert!(elapsed_ms < 300, "too slow: {elapsed_ms}ms");
        SLEEP_ACCURATE_DONE.store(true, Ordering::SeqCst);
    }

    #[test]
    fn mock_reader_sleep_duration_is_accurate() {
        run_embassy(
            |s| { s.spawn(sleep_accurate_task()).expect("spawn"); },
            &SLEEP_ACCURATE_DONE,
        );
    }
}
