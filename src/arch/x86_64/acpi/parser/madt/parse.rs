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

use core::mem;
use core::ptr;

use super::super::state::TableRegistry;
use super::entries::*;
use super::x2apic::*;
use crate::arch::x86_64::acpi::tables::madt::*;
use crate::arch::x86_64::acpi::tables::{MAX_TABLE_BYTES, SIG_MADT};

pub fn parse_madt(registry: &mut TableRegistry) {
    let addr = match registry.tables.get(&SIG_MADT) {
        Some(&a) => a,
        None => return,
    };
    let addr = match super::super::phys::directmap(addr) {
        Some(v) => v,
        None => return,
    };

    unsafe {
        let madt = ptr::read_volatile(addr as *const Madt);

        registry.data.lapic_address = madt.local_apic_address as u64;
        registry.data.has_legacy_pics = madt.has_legacy_pics();

        let madt_end = addr + (madt.header.length as u64).min(MAX_TABLE_BYTES);
        let mut entry_ptr = addr + mem::size_of::<Madt>() as u64;

        while entry_ptr + 2 <= madt_end {
            let header = ptr::read_volatile(entry_ptr as *const MadtEntryHeader);

            if header.length < 2 || entry_ptr + header.length as u64 > madt_end {
                break;
            }

            match header.entry_type {
                0 => parse_local_apic(registry, entry_ptr, header.length),
                1 => parse_ioapic(registry, entry_ptr, header.length),
                2 => parse_interrupt_override(registry, entry_ptr, header.length),
                4 => parse_local_apic_nmi(registry, entry_ptr, header.length),
                5 => parse_lapic_override(registry, entry_ptr, header.length),
                9 => parse_x2apic(registry, entry_ptr, header.length),
                10 => parse_x2apic_nmi(registry, entry_ptr, header.length),
                _ => {}
            }

            entry_ptr += header.length as u64;
        }
    }
}
