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

// The driver is no_std and uses `alloc`; link it so the included sources
// (the supplicant's Vec replies) resolve when proven on the host.
extern crate alloc;

#[path = "../../capsule_driver_iwlwifi/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_iwlwifi/src/firmware/load.rs"]
pub mod load;
#[path = "../../capsule_driver_iwlwifi/src/regs.rs"]
pub mod regs;

// The 802.11 frame layer: pure IEEE encoding, no hardware, so it is checked
// exactly rather than modeled. `src/dot11/mod.rs` pulls in the real files.
pub mod dot11;

// The host-command queue: header/sequence encoding, TFD ring math, and the
// doorbell register write.
pub mod hcmd;

// The receive path: response packet framing and the receive-ring math.
pub mod rx;

// The WPA2 key derivation: SHA-1, HMAC-SHA1, PBKDF2, the 802.11i PRF, and the
// PMK/PTK derivation, checked against RFC and IEEE known-answer vectors.
pub mod wpa;

// The EAPOL-Key handshake message layer: frame parse and MIC verification.
pub mod eapol;

// WPA2 CCMP data protection: AES-128 in CCM mode.
pub mod ccmp;

#[cfg(test)]
mod iwlwifi_tests;

#[cfg(test)]
mod hcmd_tests;

#[cfg(test)]
mod rx_tests;

#[cfg(test)]
mod harden_tests;

#[cfg(test)]
mod wpa_tests;

#[cfg(test)]
mod eapol_tests;

#[cfg(test)]
mod dot11_tests;

#[cfg(test)]
mod ccmp_tests;
#[cfg(test)]
mod data_tests;
#[path = "../../capsule_driver_iwlwifi/src/mlme/mod.rs"]
pub mod mlme;
#[cfg(test)]
mod mlme_tests;
#[cfg(test)]
mod supplicant_tests;

// The gen3 (AX210-class) firmware self-load: the context-information structure,
// the peripheral-scratch control block, the image parser, and the boot
// registers. Pure layout and parsing, checked byte-exact against the real
// so-a0-gf-a0 image and the documented struct offsets.
#[path = "../../capsule_driver_iwlwifi/src/firmware/gen3/mod.rs"]
pub mod gen3;

#[cfg(test)]
mod dram_map_tests;
#[cfg(test)]
mod gen3_image_tests;
#[cfg(test)]
mod gen3_tests;
#[cfg(test)]
mod prph_scratch_tests;
