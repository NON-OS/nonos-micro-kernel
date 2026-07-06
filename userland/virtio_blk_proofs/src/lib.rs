// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the virtio-blk driver's untrusted-input parsers.
//!
//! The request parsers take the driver state by reference, so the proofs
//! build a real `Driver` value on the host: the true struct from the driver
//! source, holding a `Queue` whose pointers are null and a `Regs` made with
//! the real constructor. Nothing is shimmed. The parsers read only the
//! capacity, and the proofs confirm it: a dereference of the queue memory
//! would crash the tests and fail the Kani harnesses.

#[path = "../../capsule_driver_virtio_blk/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_blk/src/protocol/mod.rs"]
pub mod protocol;
pub mod queue;
pub mod regs;
pub mod server;
pub mod setup;

#[cfg(test)]
mod blk_tests;

#[cfg(kani)]
mod kani_proofs;
