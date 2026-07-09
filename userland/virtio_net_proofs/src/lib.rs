// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the virtio-net driver's device-facing RX path.
//!
//! The RX queue is plain data over a DMA region, so the proofs build a real
//! `RxQueue` on the host over an allocated region and buffer area, write
//! hostile used-ring entries the way a malicious device would, and run the
//! real `take_one`. Nothing is shimmed.

#[path = "../../capsule_driver_virtio_net/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_net/src/protocol/mod.rs"]
pub mod protocol;
pub mod queue;
// Upstream take_one is unsafe without a # Safety section; kept as it ships.
#[path = "../../capsule_driver_virtio_net/src/rx.rs"]
#[allow(clippy::missing_safety_doc)]
pub mod rx;

#[cfg(test)]
mod net_tests;

#[cfg(kani)]
mod kani_proofs;
