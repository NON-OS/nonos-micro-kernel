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

use super::cpu;
use super::sys::Sys;
use crate::hardware::{detect_cpu_features, HardwareInfo};
use crate::security::SecurityContext;
use alloc::format;
use uefi::prelude::*;

pub fn gather(st: &SystemTable<Boot>, sec: &SecurityContext, hw: &HardwareInfo) -> Sys {
    let rev = st.uefi_revision();
    Sys {
        cpu: cpu::read(),
        feat: detect_cpu_features(),
        mem_bytes: hw.memory_size,
        cpu_count: hw.cpu_count,
        acpi: hw.acpi_available,
        pci: hw.pci_devices,
        storage: hw.storage_devices,
        net: hw.network_interfaces,
        gpu: hw.graphics_devices,
        secure_boot: sec.secure_boot_enabled,
        measured_boot: sec.measured_boot_active,
        rng: sec.hardware_rng_available,
        ed25519: sec.ed25519_selftest_ok,
        blake3: sec.blake3_selftest_ok,
        platform_key: sec.platform_key_verified,
        sig_db: sec.signature_database_valid,
        keys: sec.key_count,
        fw_vendor: format!("{}", st.firmware_vendor()),
        fw_rev: st.firmware_revision(),
        uefi_major: rev.major(),
        uefi_minor: rev.minor(),
        boot_ver: env!("CARGO_PKG_VERSION"),
    }
}
