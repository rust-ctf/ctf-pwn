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
