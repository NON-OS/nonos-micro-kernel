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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpDescriptor {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

impl RsdpDescriptor {
    pub const SIGNATURE: &'static [u8; 8] = b"RSD PTR ";
}

#[derive(Debug, Default, Clone)]
pub struct HardwareInfo {
    pub acpi_available: bool,
    pub rsdp_address: Option<u64>,
    pub cpu_count: usize,
    pub memory_size: u64,
    pub pci_devices: usize,
    pub storage_devices: usize,
    pub network_interfaces: usize,
    pub graphics_devices: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CpuFeatureFlags {
    pub nxe: bool,
    pub smep: bool,
    pub smap: bool,
    pub umip: bool,
}
