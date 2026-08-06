//! Host cover for the frames a mixnet write is split into, and for what a
//! short read leaves behind.
//!
//! The modules are the shipping ones, pulled in by path, so a test cannot
//! pass against a copy that has drifted from what the capsule runs.

extern crate alloc;

pub mod server;
pub mod sockets;

#[cfg(test)]
mod vectors;

#[cfg(test)]
mod residual_vectors;

pub use server::handlers::mixnet_frame;
