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
use super::srat_memory::{parse_memory_affinity, parse_x2apic_affinity};
use crate::arch::x86_64::acpi::tables::srat::SratProcessorAffinity;
use crate::arch::x86_64::acpi::tables::{Srat, SIG_SRAT};

pub fn parse_srat(registry: &mut TableRegistry) {
    let addr = match registry.tables.get(&SIG_SRAT) {
        Some(&a) => a,
        None => return,
    };
    let addr = match super::super::phys::directmap(addr) {
        Some(v) => v,
        None => return,
    };

    unsafe {
        let srat = ptr::read_volatile(addr as *const Srat);
        let srat_end = addr + srat.header.length as u64;
        let mut entry_ptr = addr + srat.entries_offset() as u64;

        while entry_ptr + 2 <= srat_end {
            let entry_type = ptr::read_volatile(entry_ptr as *const u8);
            let length = ptr::read_volatile((entry_ptr + 1) as *const u8);

            if length < 2 || entry_ptr + length as u64 > srat_end {
                break;
            }

            match entry_type {
                0 => parse_processor_affinity(registry, entry_ptr, length),
                1 => parse_memory_affinity(registry, entry_ptr, length),
                2 => parse_x2apic_affinity(registry, entry_ptr, length),
                _ => {}
            }

            entry_ptr += length as u64;
        }
    }
}

fn parse_processor_affinity(registry: &mut TableRegistry, ptr: u64, len: u8) {
    if len < mem::size_of::<SratProcessorAffinity>() as u8 {
        return;
    }
    unsafe {
        let entry = ptr::read_volatile(ptr as *const SratProcessorAffinity);
        if entry.is_enabled() {
            for proc in &mut registry.data.processors {
                if proc.apic_id == entry.apic_id as u32 {
                    proc.proximity_domain = entry.proximity_domain();
                    break;
                }
            }
        }
    }
}
