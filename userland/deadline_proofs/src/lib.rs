// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the libc monotonic Deadline. A `time` module mirroring the
//! libc path supplies a host stub for the uptime syscall so the real
//! `deadline.rs` source can be included and its pure comparison logic pinned.

pub mod time;

#[cfg(test)]
mod tests;
