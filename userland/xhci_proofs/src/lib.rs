// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the xHCI driver's TRB layer.
//!
//! Every command and transfer the driver issues, and every event the
//! controller returns, crosses the ring as a 16-byte TRB. The accessors that
//! read device-written events (completion code, slot id, cycle, type) and
//! the builders that encode control transfers must match the xHCI
//! specification bit for bit: a wrong shift silently addresses the wrong
//! slot or misreads a completion. The proofs run the real TRB source against
//! the spec layouts.

#[path = "../../capsule_driver_xhci/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_xhci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_xhci/src/trb/mod.rs"]
pub mod trb;

/*
 * The controller bring-up, run against a register window. `regs` and `error`
 * are the shipping trees whole; `controller` picks the files that talk only
 * to registers, since the rings and contexts need a DMA pool the host does
 * not have.
 */
pub mod controller;
#[path = "../../capsule_driver_xhci/src/error/mod.rs"]
pub mod error;
#[path = "../../capsule_driver_xhci/src/regs/mod.rs"]
pub mod regs;

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod xhci_tests;

#[cfg(kani)]
mod kani_proofs;
