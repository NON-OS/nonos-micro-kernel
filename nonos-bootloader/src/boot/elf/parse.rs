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
use super::display::{display_elf_info, display_load_failure, display_loaded_image};
use super::extract::extract_kernel_payload;
use crate::boot::uefi::TOTAL_BOOT_STAGES;
use crate::boot::util::fatal_reset;
use crate::display::{draw_boot_progress, update_stage, StageStatus, STAGE_ELF_PARSE};
use crate::kernel_verify::CryptoVerifyResult;
use crate::loader::{load_kernel, KernelImage};
use crate::log::logger::{log_error, log_info};
use alloc::format;
use uefi::prelude::*;

pub fn run_elf_parse(
    st: &mut SystemTable<Boot>,
    data: &[u8],
    crypto: &CryptoVerifyResult,
    gop: bool,
) -> KernelImage {
    update_stage(STAGE_ELF_PARSE, StageStatus::Running);
    draw_boot_progress(8, TOTAL_BOOT_STAGES);
    let elf = extract_kernel_payload(data, st);
    display_elf_info(elf, crypto, gop);
    match load_kernel(st, elf) {
        Ok(image) => {
            log_info("loader", "kernel loaded and verified");
            update_stage(STAGE_ELF_PARSE, StageStatus::Success);
            draw_boot_progress(9, TOTAL_BOOT_STAGES);
            display_loaded_image(&image, gop);
            image
        }
        Err(e) => {
            log_error("loader", &format!("ELF parsing failed: {}", e));
            update_stage(STAGE_ELF_PARSE, StageStatus::Failed);
            display_load_failure(elf, crypto, data, gop, &e);
            fatal_reset(st, "kernel ELF parsing failed");
        }
    }
}
