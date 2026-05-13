//Modification of pingora-timeout, to allow waiting for Duration and Instnat

use futures::future::BoxFuture;
use std::time::Duration;
use tokio::time::{sleep, sleep_until, Instant};

/// Boxed sleep future.
pub type BoxSleep = BoxFuture<'static, ()>;

/// Trait for types that can produce a sleep future.
pub trait ToTimeout {
    /// Create a boxed sleep future from this timeout value.
    fn timeout(&self) -> BoxSleep;
}

/// Timeout specification as either a delay duration or an absolute deadline.
#[derive(Debug)]
pub enum PwnTimeout {
    /// A relative delay from now.
    Delay(Duration),
    /// An absolute point in time.
    Deadline(Instant),
}

impl ToTimeout for PwnTimeout {
    fn timeout(&self) -> BoxSleep {
        let sleep = match self {
            Self::Delay(delay) => sleep(*delay),
            Self::Deadline(deadline) => sleep_until(*deadline),
        };
        Box::pin(sleep)
    }
}

impl PwnTimeout {
    #[must_use] 
    /// Calculate a deadline from a delay duration.
    pub fn calculate_deadline(delay: Duration) -> Instant {
        match Instant::now().checked_add(delay) {
            Some(deadline) => deadline,
            None => Instant::now() + Self::far_away(),
        }
    }

    #[must_use] 
    /// Returns a very large duration (30 years).
    pub fn far_away() -> Duration {
        Duration::from_hours(262_800)
    }
}

impl From<Duration> for PwnTimeout {
    fn from(value: Duration) -> Self {
        Self::Delay(value)
    }
}

impl From<Instant> for PwnTimeout {
    fn from(value: Instant) -> Self {
        Self::Deadline(value)
    }
}
