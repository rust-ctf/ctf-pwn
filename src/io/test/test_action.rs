use std::time::Duration;

#[derive(Debug, Clone)]
pub enum TestAction {
    Data(Vec<u8>),
    Sleep(Duration),
}
