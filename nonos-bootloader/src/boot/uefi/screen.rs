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

use super::TOTAL_BOOT_STAGES;
use crate::display::{
    draw_boot_progress, init_boot_screen, log_hex, log_ok, log_u32, update_stage, StageStatus,
    STAGE_UEFI,
};
use uefi::prelude::*;

pub fn run_boot_screen_init(st: &mut SystemTable<Boot>, gop: bool) {
    if !gop {
        return;
    }
    init_boot_screen();
    draw_boot_progress(1, TOTAL_BOOT_STAGES);
    log_ok(b"GOP framebuffer initialized");
    update_stage(STAGE_UEFI, StageStatus::Success);
    log_hex(b"SystemTable     ", st as *const _ as u64);
    log_hex(b"BootServices    ", st.boot_services() as *const _ as u64);
    log_hex(b"RuntimeServices ", st.runtime_services() as *const _ as u64);
    log_hex(b"ConfigTable     ", st.config_table().as_ptr() as u64);
    log_u32(b"ConfigTableCount ", st.config_table().len() as u32);
    log_ok(b"boot.toml loaded");
}
