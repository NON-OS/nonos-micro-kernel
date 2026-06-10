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
use super::super::super::util::fatal_reset;
use crate::display::{
    draw_boot_progress, log_error as panel_error, log_warn, show_error_screen, update_stage,
    StageStatus, STAGE_ZK_VERIFY,
};
use crate::log::logger::{log_error, log_info};
use crate::menu::SecurityMode;
use crate::zk::BootAttestationResult;
use uefi::prelude::*;

pub fn handle_verification_failed(
    st: &mut SystemTable<Boot>,
    r: &BootAttestationResult,
    gop: bool,
    mode: SecurityMode,
) -> BootAttestationResult {
    if mode == SecurityMode::Development {
        log_info("zk", "ZK verification failed - continuing in dev mode");
        if gop {
            log_warn(b"ZK verify FAILED (dev mode continues)");
        }
        update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
        draw_boot_progress(8, TOTAL_BOOT_STAGES);
        return r.clone();
    }
    log_error("zk", "ZK verification FAILED");
    log_error("zk", r.status_message);
    update_stage(STAGE_ZK_VERIFY, StageStatus::Failed);
    if gop {
        panel_error(b"ZK verification FAILED");
        show_error_screen(r.status_message.as_bytes());
    }
    fatal_reset(st, r.status_message);
}
