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

use super::variables::{
    load_bool_variable, load_boot_timeout, load_graphics_mode, load_network_policy,
    load_security_policy,
};
use crate::config::types::BootloaderConfig;
use crate::log::logger::{log_debug, log_info};

pub fn load_bootloader_config(system_table: &mut SystemTable<Boot>) -> BootloaderConfig {
    let mut config = BootloaderConfig::default();

    system_table
        .stdout()
        .output_string(cstr16!("=== Loading Bootloader Configuration ===\r\n"))
        .unwrap_or(());

    let security_policy = {
        let rt = system_table.runtime_services();
        load_security_policy(rt)
    };
    if let Some(policy) = security_policy {
        config.security_policy = policy;
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Security policy loaded from NVRAM\r\n"))
            .unwrap_or(());
        log_info("config", "Security policy loaded from NVRAM");
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [INFO] Using default security policy\r\n"))
            .unwrap_or(());
        log_debug("config", "Using default security policy");
    }

    let network_policy = {
        let rt = system_table.runtime_services();
        load_network_policy(rt)
    };
    if let Some(policy) = network_policy {
        config.network_policy = policy;
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Network policy loaded from NVRAM\r\n"))
            .unwrap_or(());
        log_info("config", "Network policy loaded from NVRAM");
    } else {
        system_table
            .stdout()
            .output_string(cstr16!("   [INFO] Using default network policy\r\n"))
            .unwrap_or(());
        log_debug("config", "Using default network policy");
    }

    let boot_timeout = {
        let rt = system_table.runtime_services();
        load_boot_timeout(rt)
    };
    if let Some(timeout) = boot_timeout {
        config.boot_timeout_seconds = timeout;
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Boot timeout loaded from NVRAM\r\n"))
            .unwrap_or(());
        log_info("config", "Boot timeout loaded from NVRAM");
    }

    let graphics_mode = {
        let rt = system_table.runtime_services();
        load_graphics_mode(rt)
    };
    if let Some(mode) = graphics_mode {
        config.graphics_mode = mode;
        system_table
            .stdout()
            .output_string(cstr16!("   [SUCCESS] Graphics mode loaded from NVRAM\r\n"))
            .unwrap_or(());
        log_info("config", "Graphics mode loaded from NVRAM");
    }

    config.verbose_logging = {
        let rt = system_table.runtime_services();
        load_bool_variable(rt, cstr16!("NonosVerboseLogging"))
    };
    config.diagnostic_output = {
        let rt = system_table.runtime_services();
        load_bool_variable(rt, cstr16!("NonosDiagnosticOutput"))
    };

    system_table
        .stdout()
        .output_string(cstr16!("========================================\r\n"))
        .unwrap_or(());
    log_info("config", "Configuration loading completed");

    config
}
