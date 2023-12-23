use std::time::Duration;

pub trait HasTimeout {
    fn read_timeout(&self) -> Option<Duration>;
    fn set_read_timeout(&mut self, timeout: Option<Duration>);

    fn write_timeout(&self) -> Option<Duration>;
    fn set_write_timeout(&mut self, timeout: Option<Duration>);
}
