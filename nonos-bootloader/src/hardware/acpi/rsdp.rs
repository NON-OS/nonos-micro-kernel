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

use crate::hardware::types::RsdpDescriptor;
use uefi::prelude::*;

pub fn discover_acpi_rsdp(system_table: &mut SystemTable<Boot>) -> Option<u64> {
    for entry in system_table.config_table() {
        if entry.guid == uefi::table::cfg::ACPI2_GUID || entry.guid == uefi::table::cfg::ACPI_GUID {
            let rsdp_ptr = entry.address as u64;
            if validate_rsdp(rsdp_ptr) {
                return Some(rsdp_ptr);
            }
        }
    }
    None
}

fn validate_rsdp(rsdp_address: u64) -> bool {
    if rsdp_address == 0 {
        return false;
    }
    unsafe {
        let rsdp = &*(rsdp_address as *const RsdpDescriptor);
        if &rsdp.signature != RsdpDescriptor::SIGNATURE {
            return false;
        }
        let bytes = core::slice::from_raw_parts(rsdp_address as *const u8, 20);
        bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b)) == 0
    }
}
