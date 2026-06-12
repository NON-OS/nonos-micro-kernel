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

use super::uefi::TOTAL_BOOT_STAGES;
use crate::display::{
    draw_boot_progress, log_hex, log_ok, log_size, update_stage, StageStatus, STAGE_HARDWARE,
};
use crate::hardware::{discover_system_hardware, HardwareInfo};
use uefi::prelude::*;

pub fn run_hardware_discovery(st: &mut SystemTable<Boot>, gop: bool) -> HardwareInfo {
    update_stage(STAGE_HARDWARE, StageStatus::Running);
    let hw = discover_system_hardware(st);
    if gop {
        if let Some(rsdp) = hw.rsdp_address {
            log_hex(b"ACPI RSDP @ ", rsdp);
        }
        log_ok(b"ACPI tables parsed");
        log_ok(b"PCI bus enumerated");
        log_size(b"MemoryMap size ", st.boot_services().memory_map_size().map_size);
    }
    update_stage(STAGE_HARDWARE, StageStatus::Success);
    draw_boot_progress(4, TOTAL_BOOT_STAGES);
    hw
}
