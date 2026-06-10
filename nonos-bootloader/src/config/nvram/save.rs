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

use uefi::prelude::*;
use uefi::{cstr16, CStr16};

use crate::config::types::BootloaderConfig;
use crate::log::logger::{log_error, log_info};

pub fn save_configuration(config: &BootloaderConfig, system_table: &mut SystemTable<Boot>) -> bool {
    system_table.stdout().output_string(cstr16!("=== Saving Configuration ===\r\n")).unwrap_or(());

    let mut save_successful = true;

    let policy_saved = {
        let rt = system_table.runtime_services();
        save_u8_variable(rt, cstr16!("NonosSecurityPolicy"), config.security_policy.to_u8())
    };
    if policy_saved {
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Security policy saved\r\n"))
            .unwrap_or(());
        log_info("config", "Security policy saved to NVRAM");
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [ERROR] Failed to save security policy\r\n"))
            .unwrap_or(());
        log_error("config", "Failed to save security policy to NVRAM");
        save_successful = false;
    }

    let network_policy_saved = {
        let rt = system_table.runtime_services();
        save_u8_variable(rt, cstr16!("NonosNetworkPolicy"), config.network_policy.to_u8())
    };
    if network_policy_saved {
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Network policy saved\r\n"))
            .unwrap_or(());
        log_info("config", "Network policy saved to NVRAM");
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [ERROR] Failed to save network policy\r\n"))
            .unwrap_or(());
        log_error("config", "Failed to save network policy to NVRAM");
        save_successful = false;
    }

    system_table.stdout().output_string(cstr16!("=============================\r\n")).unwrap_or(());

    if save_successful {
        log_info("config", "Configuration saved successfully");
    } else {
        log_error("config", "Configuration saving completed with some errors");
    }

    save_successful
}

pub fn save_u8_variable(
    rt: &uefi::table::runtime::RuntimeServices,
    name: &CStr16,
    value: u8,
) -> bool {
    let data = [value];
    let attributes = uefi::table::runtime::VariableAttributes::NON_VOLATILE
        | uefi::table::runtime::VariableAttributes::BOOTSERVICE_ACCESS
        | uefi::table::runtime::VariableAttributes::RUNTIME_ACCESS;

    rt.set_variable(name, &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE, attributes, &data)
        .is_ok()
}
