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

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::parser;

pub fn get_hpet_base() -> Option<u64> {
    parser::hpet_address()
}

pub fn get_lapic_base() -> Option<u64> {
    parser::lapic_address()
}

pub fn get_pcie_ecam(segment: u16, bus: u8) -> Option<u64> {
    for seg in parser::pcie_segments() {
        if seg.segment == segment && bus >= seg.start_bus && bus <= seg.end_bus {
            return Some(seg.base_address);
        }
    }
    None
}

pub fn get_ioapic_addresses() -> Vec<u64> {
    parser::ioapics().iter().map(|io| io.address).collect()
}

pub fn get_ioapic_for_gsi(gsi: u32) -> Option<u64> {
    for io in parser::ioapics() {
        if gsi >= io.gsi_base && gsi < io.gsi_base + 24 {
            return Some(io.address);
        }
    }
    None
}

pub fn processor_count() -> usize {
    parser::processors().len()
}

pub fn enabled_processor_count() -> usize {
    parser::processors().iter().filter(|p| p.enabled).count()
}

pub fn has_legacy_pics() -> bool {
    parser::has_legacy_pics().unwrap_or(true)
}

// True when the platform has an i8042 PS/2 controller. Defaults to true
// when the FADT is unavailable or unparsed so we never drop the i8042 on
// firmware that a real controller (QEMU and legacy hardware) depends on.
pub fn has_8042() -> bool {
    parser::has_8042().unwrap_or(true)
}

pub fn numa_domains() -> Vec<u32> {
    let mut domains = Vec::new();
    for region in parser::numa_regions() {
        if !domains.contains(&region.proximity_domain) {
            domains.push(region.proximity_domain);
        }
    }
    domains.sort();
    domains
}
