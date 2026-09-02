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

//! The lines bring-up is entitled to print, kept apart from the decision that
//! selects one. Each states what the hardware acknowledged and nothing more:
//! an operator reading the boot log should be able to tell whether this
//! machine confines DMA without reading the source.

use super::message::reason;
use crate::arch::x86_64::iommu::globals::page_levels;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::fault::drain_faults;
use crate::arch::x86_64::iommu::unit::probe::unit_count;
use crate::sys::serial;

pub(super) fn not_built_in() {
    serial::println(b"[VT-D] enforcement not built in; DMA is unrestricted");
}

/// Identity mapping does not confine a device that was enumerated; what it
/// buys is that anything absent from the enumeration is denied. Said for one
/// unit only, because one unit is all bring-up programs: where firmware
/// reported several, devices behind the others still reach memory directly
/// and an operator has to know that before trusting the machine.
pub(super) fn enabled(assigned: usize) {
    serial::print(b"[VT-D] translation enabled, levels=");
    serial::print_hex(page_levels().unwrap_or(0) as u64);
    serial::print(b" devices=");
    serial::print_hex(assigned as u64);
    serial::println(b"");
    serial::println(b"[VT-D] enumerated devices identity mapped; others denied");

    let units = unit_count();
    if units > 1 {
        serial::print(b"[VT-D] WARNING units=");
        serial::print_hex(units as u64);
        serial::println(b"; only the first is programmed, the rest are unrestricted");
    }

    // Anything recorded before this point came from firmware's own tables and
    // describes a machine we no longer run.
    drain_faults();
}

pub(super) fn failed(e: VtdError) {
    serial::print(b"[VT-D] bring-up failed (");
    serial::print(reason(e));
    serial::println(b"); DMA is unrestricted");
}
