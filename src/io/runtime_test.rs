//! Shared test macro for all runtimes.
//!
//! Generates the correct test attribute and executor wrapper for
//! whichever runtime is active, including embassy.

/// Generate a test function with the correct runtime executor.
///
/// Usage:
/// ```ignore
/// runtime_test!(my_test_name, {
///     Timer::sleep(Duration::from_millis(10)).await;
///     assert!(true);
/// });
/// ```
macro_rules! runtime_test {
    ($name:ident, $body:block) => {
        #[cfg(feature = "runtime-tokio")]
        #[tokio::test]
        async fn $name() $body

        #[cfg(feature = "runtime-async-std")]
        #[async_std::test]
        async fn $name() $body

        #[cfg(feature = "runtime-smol")]
        #[test]
        fn $name() {
            smol::block_on(async $body);
        }

        #[cfg(feature = "runtime-embassy")]
        mod $name {
            use super::*;

            type __BoxFut = core::pin::Pin<Box<dyn core::future::Future<Output = ()> + 'static>>;

            static __TASK: embassy_executor::raw::TaskStorage<__BoxFut> =
                embassy_executor::raw::TaskStorage::new();
            static __DONE: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);

            #[test]
            fn test() {
                use std::sync::atomic::Ordering;

                __DONE.store(false, Ordering::SeqCst);

                std::thread::spawn(|| {
                    let executor: &'static mut embassy_executor::Executor =
                        Box::leak(Box::new(embassy_executor::Executor::new()));
                    executor.run(|spawner| {
                        let token = __TASK.spawn(|| -> __BoxFut {
                            Box::pin(async {
                                { $body }
                                __DONE.store(true, Ordering::SeqCst);
                            })
                        });
                        spawner.spawn(token).expect(concat!(
                            "failed to spawn: ", stringify!($name)
                        ));
                    });
                });

                let deadline = std::time::Instant::now()
                    + core::time::Duration::from_secs(5);
                while !__DONE.load(Ordering::SeqCst) {
                    assert!(
                        std::time::Instant::now() <= deadline,
                        concat!(stringify!($name), " timed out"),
                    );
                    std::thread::sleep(core::time::Duration::from_millis(5));
                }
            }
        }
    };
}

pub(crate) use runtime_test;
