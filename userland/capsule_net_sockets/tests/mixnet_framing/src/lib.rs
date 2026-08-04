//! Host cover for the frame a mixnet write is split into.
//!
//! The module is the shipping one, pulled in by path, so a test cannot pass
//! against a copy that has drifted from what the capsule runs.

extern crate alloc;

#[path = "../../../src/server/handlers/mixnet_frame/mod.rs"]
pub mod mixnet_frame;

#[cfg(test)]
mod vectors;
