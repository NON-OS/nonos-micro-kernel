// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the e1000 driver's device-facing descriptor
//! rings.
//!
//! The RX and TX rings are plain data over DMA memory, so the proofs build
//! real rings over host arrays of the real `repr(C)` descriptor layout,
//! write hostile device-side fields, and run the real `consume` and `post`.
//! Nothing is shimmed.

#[path = "../../capsule_driver_e1000/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_e1000/src/protocol/mod.rs"]
pub mod protocol;
// Upstream descriptor accessors are unsafe fns without a # Safety section;
// kept as they ship.
pub mod init;
#[path = "../../capsule_driver_e1000/src/queue/mod.rs"]
pub mod queue;
#[path = "../../capsule_driver_e1000/src/regs.rs"]
pub mod regs;
pub mod setup;

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod e1000_tests;

#[cfg(kani)]
mod kani_proofs;
