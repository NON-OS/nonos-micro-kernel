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

use uefi::prelude::*;

use crate::display::{draw_boot_progress, show_crypto_verification, update_stage};
use crate::display::{BootCryptoState, StageStatus, STAGE_ED25519_VERIFY};
use crate::kernel_verify::CryptoVerifyResult;
use crate::menu::SecurityMode;

use super::super::uefi::TOTAL_BOOT_STAGES;
use super::extract::extract_signature_for_display;
use super::hash::compute_hash;
use super::rollback::check_rollback;
use super::signature::verify_signature;

pub fn run_crypto_verification(
    st: &mut SystemTable<Boot>,
    data: &[u8],
    gop: bool,
    mode: SecurityMode,
) -> (CryptoVerifyResult, BootCryptoState) {
    let mut state = BootCryptoState::new();
    let result = compute_hash(st, data, &mut state, gop);
    update_stage(STAGE_ED25519_VERIFY, StageStatus::Running);
    draw_boot_progress(6, TOTAL_BOOT_STAGES);
    extract_signature_for_display(data, &mut state, gop);
    state.signature_valid = Some(result.signature_valid);
    if gop {
        show_crypto_verification(&state);
    }
    verify_signature(st, &result, mode, gop);
    check_rollback(st, data, mode, gop);
    draw_boot_progress(7, TOTAL_BOOT_STAGES);
    (result, state)
}
