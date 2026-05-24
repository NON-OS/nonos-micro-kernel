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

use core::arch::asm;

pub fn virtual_offset() -> u64 {
    let offset: u64;
    unsafe {
        asm!("mrs {}, cntvoff_el2", out(reg) offset);
    }
    offset
}

pub fn physical_to_virtual(phys_count: u64) -> u64 {
    phys_count.wrapping_sub(virtual_offset())
}

pub fn virtual_to_physical(virt_count: u64) -> u64 {
    virt_count.wrapping_add(virtual_offset())
}
