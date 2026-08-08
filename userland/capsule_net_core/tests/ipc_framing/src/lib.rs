//! Host cover for the framing net.core reads requests through.
//!
//! The modules are the shipping ones, pulled in by path, so a test cannot
//! pass against a copy that has drifted from what the capsule runs.

pub mod protocol;
pub mod server;

#[cfg(test)]
mod vectors;
