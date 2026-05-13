use std::time::Duration;

/// Actions that a test reader can simulate.
#[derive(Debug, Clone)]
pub enum TestAction {
    /// Return the given bytes.
    Data(Vec<u8>),
    /// Sleep for the given duration before continuing.
    Sleep(Duration),
}
