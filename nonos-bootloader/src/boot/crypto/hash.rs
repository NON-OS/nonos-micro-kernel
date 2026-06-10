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

use crate::display::{animate_hash_reveal, draw_boot_progress, log_hash, log_ok};
use crate::display::{
    show_crypto_verification, update_stage, BootCryptoState, StageStatus, STAGE_BLAKE3_HASH,
};
use crate::kernel_verify::{verify_kernel_crypto, CryptoVerifyResult};
use crate::security::{audit, set_kernel_measurement, AuditEvent};

use super::super::uefi::TOTAL_BOOT_STAGES;
use super::super::util::micro_delay;

pub fn compute_hash(
    st: &mut SystemTable<Boot>,
    data: &[u8],
    state: &mut BootCryptoState,
    gop: bool,
) -> CryptoVerifyResult {
    update_stage(STAGE_BLAKE3_HASH, StageStatus::Running);
    draw_boot_progress(5, TOTAL_BOOT_STAGES);
    let result = verify_kernel_crypto(data, st);
    set_kernel_measurement(result.kernel_hash_full);
    audit(AuditEvent::HashComputed, 0, b"kernel hash");
    state.kernel_hash.copy_from_slice(&result.kernel_hash_full);
    if gop {
        animate_hash_display(state);
    }
    update_stage(STAGE_BLAKE3_HASH, StageStatus::Success);
    draw_boot_progress(6, TOTAL_BOOT_STAGES);
    if gop {
        log_ok(b"BLAKE3-256 hash computed");
        log_hash(b"BLAKE3 ", &result.kernel_hash_full);
    }
    result
}

fn animate_hash_display(state: &BootCryptoState) {
    for _ in 0..32 {
        animate_hash_reveal();
        show_crypto_verification(state);
        micro_delay();
    }
}
