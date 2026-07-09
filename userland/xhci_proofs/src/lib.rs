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
// Upstream LinkTrbBuilder has new() without Default; kept as it ships.
#[path = "../../capsule_driver_xhci/src/trb/mod.rs"]
#[allow(clippy::new_without_default)]
pub mod trb;

#[cfg(test)]
mod xhci_tests;

#[cfg(kani)]
mod kani_proofs;
