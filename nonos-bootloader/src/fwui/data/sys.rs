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

use super::cpu::CpuId;
use crate::hardware::CpuFeatureFlags;
use alloc::string::String;

pub struct Sys {
    pub cpu: CpuId,
    pub feat: CpuFeatureFlags,
    pub mem_bytes: u64,
    pub cpu_count: usize,
    pub acpi: bool,
    pub pci: usize,
    pub storage: usize,
    pub net: usize,
    pub gpu: usize,
    pub secure_boot: bool,
    pub measured_boot: bool,
    pub rng: bool,
    pub ed25519: bool,
    pub blake3: bool,
    pub platform_key: bool,
    pub sig_db: bool,
    pub keys: usize,
    pub fw_vendor: String,
    pub fw_rev: u32,
    pub uefi_major: u16,
    pub uefi_minor: u16,
    pub boot_ver: &'static str,
}

impl Sys {
    pub fn mem_mib(&self) -> u64 {
        self.mem_bytes / (1024 * 1024)
    }
}
