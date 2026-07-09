// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB mass-storage driver's untrusted-input
//! parsers.

#[path = "../../capsule_driver_usb_msc/src/bot/mod.rs"]
pub mod bot;
#[path = "../../capsule_driver_usb_msc/src/descriptors/mod.rs"]
pub mod descriptors;
#[path = "../../capsule_driver_usb_msc/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_usb_msc/src/scsi/mod.rs"]
pub mod scsi;

#[cfg(test)]
mod msc_tests;

#[cfg(kani)]
mod kani_proofs;
