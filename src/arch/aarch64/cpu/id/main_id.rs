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

pub fn main_id() -> u64 {
    let midr: u64;
    unsafe {
        asm!("mrs {}, midr_el1", out(reg) midr, options(nostack));
    }
    midr
}

pub fn implementer() -> u8 {
    ((main_id() >> 24) & 0xFF) as u8
}

pub fn variant() -> u8 {
    ((main_id() >> 20) & 0xF) as u8
}

pub fn architecture() -> u8 {
    ((main_id() >> 16) & 0xF) as u8
}

pub fn part_number() -> u16 {
    ((main_id() >> 4) & 0xFFF) as u16
}

pub fn revision() -> u8 {
    (main_id() & 0xF) as u8
}
