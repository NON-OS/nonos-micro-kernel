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

use super::super::uefi::TOTAL_BOOT_STAGES;
use super::super::util::mini_delay;
use super::{attestation::generate_boot_attestation, crypto::build_crypto_handoff};
use super::{
    display::{show_completion_status, show_handoff_status},
    entropy::collect_entropy,
    params::HandoffParams,
};
use crate::display::{
    draw_boot_progress, update_stage, StageStatus, STAGE_COMPLETE, STAGE_HANDOFF,
};
use crate::firmware::get_firmware_handoff;
use crate::handoff::exit_and_jump;
use crate::loader::KernelImage;
use crate::log::logger::log_info;
use crate::security::{audit, seal_audit_log, AuditEvent};
use uefi::prelude::*;

pub fn run_handoff_prepare(
    st: SystemTable<Boot>,
    ki: &KernelImage,
    p: HandoffParams,
    gop: bool,
) -> ! {
    let mut st = st;
    log_info("handoff", "starting handoff preparation");
    update_stage(STAGE_HANDOFF, StageStatus::Running);
    draw_boot_progress(10, TOTAL_BOOT_STAGES);
    let rng_seed = collect_entropy(&mut st, gop);
    let (crypto_handoff, firmware_handoff) = (build_crypto_handoff(&p), get_firmware_handoff());
    let _quote = generate_boot_attestation(&rng_seed, gop);
    if gop {
        show_handoff_status(&rng_seed);
    }
    update_stage(STAGE_HANDOFF, StageStatus::Success);
    update_stage(STAGE_COMPLETE, StageStatus::Success);
    draw_boot_progress(TOTAL_BOOT_STAGES, TOTAL_BOOT_STAGES);
    if gop {
        show_completion_status(ki);
    }
    mini_delay();
    audit(AuditEvent::ExitBootServices, 0, b"handoff");
    seal_audit_log();
    log_info("handoff", "transferring control to kernel");
    exit_and_jump(st, ki, None, crypto_handoff, firmware_handoff, rng_seed, p.tpm_measured);
}
