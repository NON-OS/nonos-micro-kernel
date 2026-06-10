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

extern crate alloc;
use super::super::uefi::TOTAL_BOOT_STAGES;
use super::super::util::fatal_reset;
use super::display::display_kernel_info;
use crate::display::{
    draw_boot_progress, log_error as panel_error, show_error_screen, update_stage, StageStatus,
    STAGE_KERNEL_LOAD,
};
use crate::loader::load_kernel_binary;
use crate::log::logger::{log_error, log_info};
use alloc::vec::Vec;
use uefi::prelude::*;

pub fn run_kernel_load(st: &mut SystemTable<Boot>, gop: bool) -> Vec<u8> {
    update_stage(STAGE_KERNEL_LOAD, StageStatus::Running);
    draw_boot_progress(4, TOTAL_BOOT_STAGES);
    match load_kernel_binary(st) {
        Ok(data) => {
            log_info("loader", "kernel binary loaded");
            update_stage(STAGE_KERNEL_LOAD, StageStatus::Success);
            draw_boot_progress(5, TOTAL_BOOT_STAGES);
            if gop {
                display_kernel_info(&data);
            }
            data
        }
        Err(_) => {
            log_error("loader", "kernel load failed");
            update_stage(STAGE_KERNEL_LOAD, StageStatus::Failed);
            if gop {
                panel_error(b"FATAL: kernel.bin not found");
                show_error_screen(b"Kernel not found");
            }
            fatal_reset(st, "kernel not found");
        }
    }
}
