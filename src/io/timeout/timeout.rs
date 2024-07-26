//Modification of pingora-timeout, to allow waiting for Duration and Instnat

use futures::future::BoxFuture;
use std::time::Duration;
use tokio::time::{sleep, sleep_until, Instant};

pub type BoxSleep = BoxFuture<'static, ()>;
pub trait ToTimeout {
    fn timeout(&self) -> BoxSleep;
}

pub enum PwnTimeout {
    Delay(Duration),
    Deadline(Instant),
}

impl ToTimeout for PwnTimeout {
    fn timeout(&self) -> BoxSleep {
        let sleep = match self {
            PwnTimeout::Delay(delay) => sleep(*delay),
            PwnTimeout::Deadline(deadline) => sleep_until(*deadline),
        };
        Box::pin(sleep)
    }
}

impl PwnTimeout {
    pub fn calculate_deadline(delay: Duration) -> Instant {
        match Instant::now().checked_add(delay) {
            Some(deadline) => deadline,
            //Instant::far_future()
            None => Instant::now() + Duration::from_secs(86400 * 365 * 30),
        }
    }
}

impl From<Duration> for PwnTimeout {
    fn from(value: Duration) -> Self {
        PwnTimeout::Delay(value)
    }
}

impl From<Instant> for PwnTimeout {
    fn from(value: Instant) -> Self {
        PwnTimeout::Deadline(value)
    }
}
