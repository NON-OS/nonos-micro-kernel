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
use super::acpi::{discover_acpi_rsdp, get_cpu_count_from_acpi};
use super::cpu::detect_cpu_features;
use super::devices::{enumerate_graphics, enumerate_network, enumerate_pci, enumerate_storage};
use super::display::display_hardware_summary;
use super::memory::discover_memory_size;
use super::types::HardwareInfo;
use crate::log::logger::log_info;
use alloc::format;
use uefi::prelude::*;

pub fn discover_system_hardware(system_table: &mut SystemTable<Boot>) -> HardwareInfo {
    let mut hw = HardwareInfo::default();
    hw.rsdp_address = discover_acpi_rsdp(system_table);
    hw.acpi_available = hw.rsdp_address.is_some();
    hw.memory_size = discover_memory_size(system_table);
    log_info("memory", &format!("Total RAM: {} MiB", hw.memory_size / (1024 * 1024)));
    hw.cpu_count = hw.rsdp_address.map(get_cpu_count_from_acpi).unwrap_or(1);
    hw.storage_devices = enumerate_storage(system_table);
    hw.network_interfaces = enumerate_network(system_table);
    hw.graphics_devices = enumerate_graphics(system_table);
    hw.pci_devices = enumerate_pci(system_table);
    let cpu_flags = detect_cpu_features();
    log_info(
        "cpu",
        &format!(
            "NXE={} SMEP={} SMAP={} UMIP={}",
            cpu_flags.nxe, cpu_flags.smep, cpu_flags.smap, cpu_flags.umip
        ),
    );
    display_hardware_summary(&hw, system_table);
    hw
}
