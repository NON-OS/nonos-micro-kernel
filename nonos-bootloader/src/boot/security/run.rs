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

use alloc::format;
use uefi::prelude::*;

use crate::display::{draw_boot_progress, update_stage, StageStatus, STAGE_SECURITY};
use crate::log::logger::{log_info, log_warn};
use crate::security::{tpm_counter_selftest, SecurityContext, RC_NO_TPM};

use super::super::uefi::TOTAL_BOOT_STAGES;
use super::hardware::verify_hardware_requirements;
use super::init::init_security_primitives;
use super::platform::{init_subsystems, verify_platform};
use super::policy::{enforce_policy, verify_chain};

pub fn run_security_checks(st: &mut SystemTable<Boot>, gop: bool) -> SecurityContext {
    update_stage(STAGE_SECURITY, StageStatus::Running);
    draw_boot_progress(2, TOTAL_BOOT_STAGES);
    init_security_primitives();
    let hw_caps = verify_hardware_requirements(st, gop);
    verify_platform(&hw_caps, gop);
    let security = init_subsystems(st, gop);
    enforce_policy(&security, st, gop);
    verify_chain(&security, st);
    match tpm_counter_selftest(st.boot_services()) {
        Ok((first, second)) => {
            log_info("tpm-counter", &format!("monotonic v1={} v2={}", first, second))
        }
        Err(rc) if rc == RC_NO_TPM => log_info("tpm-counter", "no TPM present, selftest skipped"),
        Err(rc) => log_warn("tpm-counter", &format!("selftest rc=0x{:08x}", rc)),
    }
    update_stage(STAGE_SECURITY, StageStatus::Success);
    draw_boot_progress(3, TOTAL_BOOT_STAGES);
    security
}
