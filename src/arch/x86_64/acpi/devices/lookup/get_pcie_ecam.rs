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

use crate::arch::x86_64::acpi::parser;

pub fn get_pcie_ecam(segment: u16, bus: u8) -> Option<u64> {
    for seg in parser::pcie_segments() {
        if seg.segment == segment && bus >= seg.start_bus && bus <= seg.end_bus {
            return Some(seg.base_address);
        }
    }
    None
}
