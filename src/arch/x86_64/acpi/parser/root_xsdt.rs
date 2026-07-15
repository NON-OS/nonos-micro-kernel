// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

use super::state::TableRegistry;
use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};
use crate::arch::x86_64::acpi::tables::{sdt_entry_count, SdtHeader, SIG_SSDT, SIG_XSDT};
use core::{mem, ptr};

pub fn parse_xsdt(registry: &mut TableRegistry, addr: u64) -> AcpiResult<()> {
    let addr = super::phys::directmap(addr).ok_or(AcpiError::InvalidXsdtSignature)?;
    unsafe {
        let header = ptr::read_volatile(addr as *const SdtHeader);
        if header.signature != SIG_XSDT {
            return Err(AcpiError::InvalidXsdtSignature);
        }
        if !header.validate_checksum(addr as *const u8) {
            return Err(AcpiError::XsdtChecksumFailed);
        }
        let entry_count = sdt_entry_count(header.length as usize, mem::size_of::<SdtHeader>(), 8);
        let entries_ptr = (addr as usize + mem::size_of::<SdtHeader>()) as *const u64;
        for i in 0..entry_count {
            let entry_addr = ptr::read_volatile(entries_ptr.add(i));
            if entry_addr != 0 {
                if let Some(hdr) = super::phys::directmap(entry_addr) {
                    let table_header = ptr::read_volatile(hdr as *const SdtHeader);
                    if table_header.signature == SIG_SSDT {
                        registry.ssdt_addresses.push(entry_addr);
                    }
                    registry.tables.insert(table_header.signature, entry_addr);
                }
            }
        }
    }
    Ok(())
}
