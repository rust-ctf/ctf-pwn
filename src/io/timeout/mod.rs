//! Timeout types and utilities for async operations.

mod error;
mod ext;
#[expect(clippy::module_inception, reason = "timeout is the natural name for this module")]
mod timeout;

pub use error::*;
pub use ext::*;
pub use timeout::*;
