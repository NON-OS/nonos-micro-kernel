// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for driver request parsers over untrusted input.

#[path = "../../capsule_driver_ahci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_ahci/src/constants/mod.rs"]
pub mod constants;
pub mod server;

#[cfg(test)]
mod ahci_tests;

#[cfg(kani)]
mod kani_proofs;
