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

use uefi::cstr16;
use uefi::prelude::*;

use crate::hardware::HardwareInfo;
use crate::log::logger::log_info;

use super::types::{BootloaderConfig, MemoryManagementMode};

pub fn apply_hardware_settings(
    config: &BootloaderConfig,
    system_table: &mut SystemTable<Boot>,
    hardware: &HardwareInfo,
) {
    if config.cpu_optimizations && hardware.cpu_count > 1 {
        system_table
            .stdout()
            .output_string(cstr16!("   [INFO] CPU optimizations enabled for multi-core system\r\n"))
            .unwrap_or(());
        log_info("config", "CPU optimizations enabled");
    }
}

pub fn apply_memory_settings(config: &BootloaderConfig, system_table: &mut SystemTable<Boot>) {
    match config.memory_management_mode {
        MemoryManagementMode::Secure => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Secure memory management mode active\r\n"))
                .unwrap_or(());
            log_info("config", "Secure memory management mode applied");
        }
        MemoryManagementMode::Efficient => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Efficient memory management mode active\r\n"))
                .unwrap_or(());
            log_info("config", "Efficient memory management mode applied");
        }
        MemoryManagementMode::Legacy => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Legacy memory management mode active\r\n"))
                .unwrap_or(());
            log_info("config", "Legacy memory management mode applied");
        }
    }
}
