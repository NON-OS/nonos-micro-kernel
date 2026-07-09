// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the NVMe driver's untrusted-input parsers.

#[path = "../../capsule_driver_nvme/src/protocol/mod.rs"]
pub mod protocol;
pub mod admin;
pub mod nvm;
pub mod server;

#[cfg(test)]
mod nvme_tests;

#[cfg(kani)]
mod kani_proofs;
