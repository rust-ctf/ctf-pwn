//! Protocol-specific pipe implementations.

mod process;
mod tcp;
pub use process::*;
pub use tcp::*;
