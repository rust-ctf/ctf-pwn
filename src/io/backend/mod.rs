//! I/O backends for [`Session`](crate::session::Session).
//!
//! Provides [`Split`] for separate reader/writer halves and [`Duplex`]
//! for a single bidirectional object.

mod split;
mod duplex;

pub use duplex::Duplex;
pub use split::Split;
