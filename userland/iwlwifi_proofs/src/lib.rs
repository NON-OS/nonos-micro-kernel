// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the iwlwifi firmware-load path.
//!
//! The FH (Flow Handler) firmware-into-device transfer is a fixed sequence of
//! MMIO register writes defined by the hardware spec. These proofs include the
//! real `load_sections` / `load_firmware_chunk` code and run it against a
//! modeled device (`MockMmio`) that records every write and reports the FH
//! transfer as complete. The proofs then assert the exact register sequence,
//! values, and addressing the driver programs, so the code is validated
//! against the documented spec without any hardware.
//!
//! This validates the driver's register sequence. It does not and cannot
//! assert that a real Intel chip responds correctly; that still needs silicon.

#[path = "../../capsule_driver_iwlwifi/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_iwlwifi/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_iwlwifi/src/firmware/load.rs"]
pub mod load;

#[cfg(test)]
mod iwlwifi_tests;
