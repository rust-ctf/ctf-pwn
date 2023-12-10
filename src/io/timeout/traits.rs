use std::time::Duration;

pub trait HasTimeout {
    fn timeout(&self) -> Option<Duration>;
    fn set_timeout(&mut self, timeout: Option<Duration>);
}
