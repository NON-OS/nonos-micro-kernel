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

use core::ptr;

use super::super::state::TableRegistry;
use crate::arch::x86_64::acpi::data::PcieSegment;
use crate::arch::x86_64::acpi::tables::{Mcfg, McfgEntry, SIG_MCFG};

pub fn parse_mcfg(registry: &mut TableRegistry) {
    let addr = match registry.tables.get(&SIG_MCFG) {
        Some(&a) => a,
        None => return,
    };
    let addr = match super::super::phys::directmap(addr) {
        Some(v) => v,
        None => return,
    };

    unsafe {
        let mcfg = ptr::read_volatile(addr as *const Mcfg);
        let entry_count = mcfg.entry_count();
        let entries_ptr = (addr + mcfg.entries_offset() as u64) as *const McfgEntry;

        for i in 0..entry_count {
            let entry = ptr::read_volatile(entries_ptr.add(i));
            registry.data.pcie_segments.push(PcieSegment {
                base_address: entry.base_address,
                segment: entry.segment_group,
                start_bus: entry.start_bus,
                end_bus: entry.end_bus,
            });
        }
    }
}
