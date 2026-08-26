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

use crate::config::load_bootloader_config;
use crate::display::{init_boot_screen, init_gop, report_gop_mode};
use crate::firmware::detect_firmware_quirks;
use crate::log::logger::{init_logger, log_info};
use uefi::prelude::*;

pub struct UefiInitResult {
    pub gop_available: bool,
}

pub fn run_uefi_init(st: &mut SystemTable<Boot>) -> UefiInitResult {
    let gop_available = init_gop(st);
    init_logger(st);
    log_info("boot", "UEFI services initialized");
    report_gop_mode();
    let _config = load_bootloader_config(st);
    let _quirks = detect_firmware_quirks(st);
    log_info("firmware", "detected firmware quirks");
    if gop_available {
        init_boot_screen();
    }
    UefiInitResult { gop_available }
}
