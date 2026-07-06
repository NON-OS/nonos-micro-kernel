// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the RTL8139 driver's RX ring primitives.
//!
//! The RTL8139 receives into a byte ring the device fills with per-packet
//! headers: a status word and a raw length, both device-chosen, and the
//! driver walks the ring by offset arithmetic. The ring readers and the
//! wrapping copy are the memory-safety core of that walk: every access they
//! make must land inside the ring no matter what offsets and lengths the
//! device induces. The proofs run the real primitives over a host ring.
//! The gate above them (`read_frame`) rejects bad status, short raw
//! lengths, and frames larger than Ethernet or the caller's buffer before
//! any copy; it reaches the hardware through PIO and IRQ syscalls and so is
//! not host-runnable, but every access it performs goes through the
//! primitives proven here.

#[path = "../../capsule_driver_rtl8139/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_rtl8139/src/protocol/mod.rs"]
pub mod protocol;
pub mod ring;

#[cfg(test)]
mod rtl_tests;

#[cfg(kani)]
mod kani_proofs;
