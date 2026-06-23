//! Shared test macro for std-based runtimes (tokio, async-std, smol).
//!
//! Generates the correct test attribute and executor wrapper for
//! whichever runtime is active.

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
    };
}

pub(crate) use runtime_test;
