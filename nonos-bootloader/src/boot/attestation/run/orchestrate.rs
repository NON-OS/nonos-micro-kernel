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
use super::super::enforce::enforce_zk_binding;
use super::{
    failed::handle_verification_failed,
    no_proof::handle_no_proof,
    success::{display_success, update_crypto_state},
};
use crate::display::{
    draw_boot_progress, update_stage, BootCryptoState, StageStatus, STAGE_ZK_VERIFY,
};
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
    if !has_zk_proof(data) {
        return handle_no_proof(st, gop, mode);
    }
    let zk_result = verify_boot_attestation(data);
    if !zk_result.zk_verified {
        return handle_verification_failed(st, &zk_result, gop, mode);
    }
    enforce_zk_binding(st, &zk_result, data, kh, gop);
    update_crypto_state(cs, &zk_result);
    display_success(st, &zk_result, kh, gop, tpm);
    log_info("zk", "ZK attestation VERIFIED with kernel binding");
    update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
    draw_boot_progress(8, TOTAL_BOOT_STAGES);
    zk_result
}
