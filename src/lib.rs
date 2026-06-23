//! CTF pwn utilities for Rust.
//!
//! Runtime-agnostic async I/O primitives for interacting with CTF challenge services,
//! hardware targets, and local binaries.

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;
