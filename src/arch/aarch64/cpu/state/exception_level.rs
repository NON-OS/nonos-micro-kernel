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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionLevel {
    El0,
    El1,
    El2,
    El3,
}

pub fn current_el() -> ExceptionLevel {
    let el: u64;
    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) el, options(nostack));
    }
    match (el >> 2) & 0x3 {
        0 => ExceptionLevel::El0,
        1 => ExceptionLevel::El1,
        2 => ExceptionLevel::El2,
        _ => ExceptionLevel::El3,
    }
}

pub fn is_el1() -> bool {
    current_el() == ExceptionLevel::El1
}

pub fn is_el2() -> bool {
    current_el() == ExceptionLevel::El2
}

pub fn is_el3() -> bool {
    current_el() == ExceptionLevel::El3
}
