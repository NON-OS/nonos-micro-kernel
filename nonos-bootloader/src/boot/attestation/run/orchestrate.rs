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

use super::super::super::uefi::TOTAL_BOOT_STAGES;
use super::super::super::zk_challenge::{clear_zk_challenge, write_zk_challenge};
use super::super::enforce::enforce_zk_binding;
use super::super::source::*;
use super::{
    failed::handle_verification_failed,
    invalid_sidecar::handle_invalid_sidecar,
    no_proof::handle_no_proof,
    require_runtime::require_runtime_source,
    success::{display_success, update_crypto_state},
};
use crate::display::*;
use crate::log::logger::log_info;
use crate::menu::SecurityMode;
use crate::zk::{has_zk_proof, verify_boot_attestation, BootAttestationResult};
use uefi::prelude::*;

pub fn run_zk_attestation(
    st: &mut SystemTable<Boot>,
    data: &[u8],
    kh: &[u8; 32],
    cs: &mut BootCryptoState,
    gop: bool,
    tpm: bool,
    mode: SecurityMode,
) -> BootAttestationResult {
    update_stage(STAGE_ZK_VERIFY, StageStatus::Running);
    draw_boot_progress(7, TOTAL_BOOT_STAGES);
    let proof_source = select_zk_proof_source(st, data);
    let proof_bytes = proof_source_bytes(&proof_source);
    if proof_source_is_invalid_sidecar(&proof_source) {
        return handle_invalid_sidecar(st, kh, gop, mode);
    }
    if let Some(result) = require_runtime_source(st, &proof_source, kh, gop, mode) {
        return result;
    }
    if !has_zk_proof(proof_bytes) {
        write_zk_challenge(st, kh);
        return handle_no_proof(st, gop, mode);
    }
    let zk_result = verify_boot_attestation(proof_bytes);
    if !zk_result.zk_verified {
        write_zk_challenge(st, kh);
        return handle_verification_failed(st, &zk_result, gop, mode);
    }
    enforce_zk_binding(st, &zk_result, proof_bytes, kh, gop);
    if proof_source_is_sidecar(&proof_source) {
        clear_zk_challenge(st);
    } else {
        write_zk_challenge(st, kh);
    }
    update_crypto_state(cs, &zk_result);
    display_success(st, &zk_result, kh, gop, tpm);
    log_info("zk", "ZK attestation VERIFIED with kernel binding");
    update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
    draw_boot_progress(8, TOTAL_BOOT_STAGES);
    zk_result
}
