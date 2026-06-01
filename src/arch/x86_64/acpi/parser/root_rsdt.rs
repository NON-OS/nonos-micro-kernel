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
use crate::arch::x86_64::acpi::tables::{SdtHeader, SIG_RSDT};
use core::{mem, ptr};

pub fn parse_rsdt(registry: &mut TableRegistry, addr: u64) -> AcpiResult<()> {
    let addr = super::phys::directmap(addr).ok_or(AcpiError::InvalidRsdtSignature)?;
    unsafe {
        let header = ptr::read_volatile(addr as *const SdtHeader);
        if header.signature != SIG_RSDT {
            return Err(AcpiError::InvalidRsdtSignature);
        }
        if !header.validate_checksum(addr as *const u8) {
            return Err(AcpiError::RsdtChecksumFailed);
        }
        let entry_count = (header.length as usize - mem::size_of::<SdtHeader>()) / 4;
        let entries_ptr = (addr as usize + mem::size_of::<SdtHeader>()) as *const u32;
        for i in 0..entry_count {
            let entry_addr = ptr::read_volatile(entries_ptr.add(i)) as u64;
            if entry_addr != 0 {
                if let Some(hdr) = super::phys::directmap(entry_addr) {
                    let table_header = ptr::read_volatile(hdr as *const SdtHeader);
                    registry.tables.insert(table_header.signature, entry_addr);
                }
            }
        }
    }
    Ok(())
}
