use core::future::Future;
use core::pin::pin;
use core::time::Duration;

use crate::io::runtime_test::runtime_test;
use super::{Timeout, Timer, TimerProvider, test_helpers};


runtime_test!(sleep_completes, {
    test_helpers::sleep_completes::<Timer>().await;
});

runtime_test!(sleep_zero_completes, {
    test_helpers::sleep_zero_completes::<Timer>().await;
});

runtime_test!(sleep_is_unpin, {
    test_helpers::sleep_is_unpin::<Timer>();
});

runtime_test!(now_is_monotonic, {
    test_helpers::now_is_monotonic::<Timer>();
});

runtime_test!(timeout_delay_into_sleep_completes, {
    test_helpers::timeout_delay_into_sleep_completes().await;
});

runtime_test!(timeout_from_duration, {
    test_helpers::timeout_from_duration();
});

runtime_test!(timeout_far_away_is_large, {
    test_helpers::timeout_far_away_is_large();
});

runtime_test!(sleep_polls_pending_then_ready, {
    let mut fut = pin!(Timer::sleep(Duration::from_millis(50)));

    let is_pending = core::future::poll_fn(|cx| {
        let result = fut.as_mut().poll(cx);
        core::task::Poll::Ready(result.is_pending())
    }).await;
    assert!(is_pending, "first poll should be Pending");

    Timer::sleep(Duration::from_millis(60)).await;

    let is_ready = core::future::poll_fn(|cx| {
        let result = fut.as_mut().poll(cx);
        core::task::Poll::Ready(result.is_ready())
    }).await;
    assert!(is_ready, "should be Ready after waiting");
});

runtime_test!(sleep_until_past_deadline_completes_immediately, {
    let past = Timer::now();
    Timer::sleep(Duration::from_millis(5)).await;
    Timer::sleep_until(past).await;
});

runtime_test!(sleep_until_future_deadline_completes, {
    let deadline = Timer::deadline(Duration::from_millis(20));
    Timer::sleep_until(deadline).await;
    assert!(Timer::now() >= deadline);
});

runtime_test!(sleep_registers_waker_correctly, {
    let mut fut = pin!(Timer::sleep(Duration::from_millis(10)));

    // First poll registers the waker
    let is_pending = core::future::poll_fn(|cx| {
        let result = fut.as_mut().poll(cx);
        core::task::Poll::Ready(result.is_pending())
    }).await;
    assert!(is_pending);

    // Wait for the sleep to expire
    Timer::sleep(Duration::from_millis(20)).await;

    // Should be ready now
    let is_ready = core::future::poll_fn(|cx| {
        let result = fut.as_mut().poll(cx);
        core::task::Poll::Ready(result.is_ready())
    }).await;
    assert!(is_ready);
});

runtime_test!(now_returns_ordered_after_sleep, {
    let before = Timer::now();
    Timer::sleep(Duration::from_millis(10)).await;
    let after = Timer::now();
    assert!(after > before);
});

runtime_test!(timeout_deadline_into_sleep_completes, {
    let deadline = Timer::deadline(Duration::from_millis(20));
    let timeout = Timeout::Deadline(deadline);
    timeout.into_sleep().await;
    assert!(Timer::now() >= deadline);
});

runtime_test!(timeout_from_instant, {
    let instant = Timer::deadline(Duration::from_millis(10));
    let timeout: Timeout = instant.into();
    assert!(matches!(timeout, Timeout::Deadline(_)));
    timeout.into_sleep().await;
});

runtime_test!(timeout_copy_produces_independent_sleeps, {
    let timeout = Timeout::Delay(Duration::from_millis(10));
    let timeout2 = timeout;
    futures::join!(timeout.into_sleep(), timeout2.into_sleep());
});

runtime_test!(shorter_sleep_wakes_before_longer, {
    let max_deadline = Timer::deadline(Duration::from_millis(100));
    let short = Timer::sleep(Duration::from_millis(20));
    let long = Timer::sleep(Duration::from_millis(200));

    futures::future::select(pin!(short), pin!(long)).await;

    assert!(Timer::now() < max_deadline, "short sleep took too long");
});

runtime_test!(timer_survives_multiple_wakes_then_fires, {
    let mut deadline = pin!(Timer::sleep(Duration::from_millis(100)));

    for _ in 0..5 {
        let tick = pin!(Timer::sleep(Duration::from_millis(10)));
        match futures::future::select(tick, &mut deadline).await {
            futures::future::Either::Left(_) => {}
            futures::future::Either::Right(_) => {
                unreachable!("deadline should not fire during early iterations");
            }
        }
    }

    deadline.await;
});

runtime_test!(timer_fires_after_other_work_completes, {
    let mut timeout = pin!(Timer::sleep(Duration::from_millis(50)));
    let mut count = 0u32;

    loop {
        let tick = pin!(Timer::sleep(Duration::from_millis(10)));
        match futures::future::select(tick, &mut timeout).await {
            futures::future::Either::Left(_) => count += 1,
            futures::future::Either::Right(_) => break,
        }
    }

    assert!(count >= 3, "expected at least 3 iterations before timeout, got {count}");
});
