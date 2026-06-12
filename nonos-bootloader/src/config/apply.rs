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
use crate::log::logger::{log_error, log_info, log_warn};
use crate::network::NetworkBootContext;
use crate::security::SecurityContext;

use super::apply_hardware::{apply_hardware_settings, apply_memory_settings};
use super::apply_network::apply_network_policy;
use super::apply_security::apply_security_policy;
use super::types::BootloaderConfig;

pub fn apply_configuration(
    config: &BootloaderConfig,
    system_table: &mut SystemTable<Boot>,
    security: &SecurityContext,
    network: &NetworkBootContext,
    hardware: &HardwareInfo,
) -> bool {
    system_table
        .stdout()
        .output_string(cstr16!("=== Applying Configuration ===\r\n"))
        .unwrap_or(());

    let mut application_successful = true;

    if !apply_security_policy(config, system_table, security) {
        system_table
            .stdout()
            .output_string(cstr16!("   [ERROR] Failed to apply security policy\r\n"))
            .unwrap_or(());
        log_error("config", "Failed to apply security policy");
        application_successful = false;
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Security policy applied\r\n"))
            .unwrap_or(());
        log_info("config", "Security policy applied successfully");
    }

    if !apply_network_policy(config, system_table, network) {
        system_table
            .stdout()
            .output_string(cstr16!("   [WARN] Network policy application had issues\r\n"))
            .unwrap_or(());
        log_warn("config", "Network policy application had issues");
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Network policy applied\r\n"))
            .unwrap_or(());
        log_info("config", "Network policy applied successfully");
    }

    apply_hardware_settings(config, system_table, hardware);

    apply_memory_settings(config, system_table);

    system_table
        .stdout()
        .output_string(cstr16!("===============================\r\n"))
        .unwrap_or(());

    if application_successful {
        log_info("config", "All configuration applied successfully");
    } else {
        log_warn("config", "Configuration application completed with some issues");
    }

    application_successful
}
