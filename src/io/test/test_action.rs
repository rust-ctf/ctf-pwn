use std::time::Duration;

pub enum TestAction {
    Data(Vec<u8>),
    Sleep(Duration),
}
