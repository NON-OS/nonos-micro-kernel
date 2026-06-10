// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::display::{log_hash, log_security, BootCryptoState};
use crate::log::logger::log_info;
use crate::security::extend_boot_measurements;
use crate::zk::BootAttestationResult;
use uefi::prelude::*;

pub fn update_crypto_state(state: &mut BootCryptoState, result: &BootAttestationResult) {
    state.zk_present = true;
    state.zk_program_hash.copy_from_slice(&result.program_hash);
    state.zk_verified = result.zk_verified;
}

pub fn display_success(
    st: &mut SystemTable<Boot>,
    r: &BootAttestationResult,
    kh: &[u8; 32],
    gop: bool,
    tpm: bool,
) {
    if gop {
        log_security(b"groth16/bls12-381 VERIFIED kernel-bound");
        log_hash(b"ZK prog ", &r.program_hash);
        log_hash(b"capsule ", &r.capsule_commitment);
    }
    if tpm {
        let sig = [0u8; 64];
        extend_boot_measurements(st, kh, &sig, &r.program_hash);
        log_info("tpm", "measurements extended");
    }
}
