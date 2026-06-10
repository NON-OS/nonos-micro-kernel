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

use super::progress::get_progress;
use crate::display::log_panel::{log_error, log_info, log_ok};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StageStatus {
    Pending,
    Running,
    Success,
    Failed,
}

static STAGE_NAMES: [&[u8]; 11] = [
    b"init",
    b"uefi",
    b"security",
    b"hardware",
    b"kernel_load",
    b"blake3",
    b"ed25519",
    b"zk_verify",
    b"elf_parse",
    b"handoff",
    b"complete",
];

pub fn update_stage(stage: u8, status: StageStatus) {
    let name =
        if (stage as usize) < STAGE_NAMES.len() { STAGE_NAMES[stage as usize] } else { b"unknown" };
    match status {
        StageStatus::Pending => {}
        StageStatus::Running => log_info(name),
        StageStatus::Success => log_ok(name),
        StageStatus::Failed => log_error(name),
    }
}

pub fn get_boot_progress_percent() -> u8 {
    let (current, total) = get_progress();
    if total == 0 {
        return 0;
    }
    ((current as u32 * 100) / total as u32) as u8
}
