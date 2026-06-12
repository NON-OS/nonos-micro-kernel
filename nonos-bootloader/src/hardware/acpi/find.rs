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

use super::tables::AcpiSdtHeader;
use crate::hardware::types::RsdpDescriptor;
const MADT_SIGNATURE: &[u8; 4] = b"APIC";

pub fn find_madt_table(rsdp_address: u64) -> u64 {
    unsafe {
        let rsdp = &*(rsdp_address as *const RsdpDescriptor);
        if rsdp.revision >= 2 && rsdp.xsdt_address != 0 {
            return find_in_xsdt(rsdp.xsdt_address);
        }
        if rsdp.rsdt_address != 0 {
            return find_in_rsdt(rsdp.rsdt_address as u64);
        }
    }
    0
}

fn find_in_xsdt(xsdt_addr: u64) -> u64 {
    unsafe {
        let hdr = &*(xsdt_addr as *const AcpiSdtHeader);
        let entries = (hdr.length as usize - 36) / 8;
        let ptrs = core::slice::from_raw_parts((xsdt_addr + 36) as *const u64, entries);
        for &addr in ptrs {
            if table_matches(addr) {
                return addr;
            }
        }
    }
    0
}

fn find_in_rsdt(rsdt_addr: u64) -> u64 {
    unsafe {
        let hdr = &*(rsdt_addr as *const AcpiSdtHeader);
        let entries = (hdr.length as usize - 36) / 4;
        let ptrs = core::slice::from_raw_parts((rsdt_addr + 36) as *const u32, entries);
        for &addr in ptrs {
            if table_matches(addr as u64) {
                return addr as u64;
            }
        }
    }
    0
}

fn table_matches(addr: u64) -> bool {
    unsafe {
        let hdr = &*(addr as *const AcpiSdtHeader);
        &hdr.signature == MADT_SIGNATURE && hdr.validate()
    }
}
