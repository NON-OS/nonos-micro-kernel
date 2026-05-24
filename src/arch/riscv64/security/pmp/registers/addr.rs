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

use super::super::error::{PmpError, PmpResult};

pub fn write_addr(index: usize, addr: u64) -> PmpResult<()> {
    match index {
        0 => unsafe { asm!("csrw pmpaddr0, {}", in(reg) addr) },
        1 => unsafe { asm!("csrw pmpaddr1, {}", in(reg) addr) },
        2 => unsafe { asm!("csrw pmpaddr2, {}", in(reg) addr) },
        3 => unsafe { asm!("csrw pmpaddr3, {}", in(reg) addr) },
        4 => unsafe { asm!("csrw pmpaddr4, {}", in(reg) addr) },
        5 => unsafe { asm!("csrw pmpaddr5, {}", in(reg) addr) },
        6 => unsafe { asm!("csrw pmpaddr6, {}", in(reg) addr) },
        7 => unsafe { asm!("csrw pmpaddr7, {}", in(reg) addr) },
        8 => unsafe { asm!("csrw pmpaddr8, {}", in(reg) addr) },
        9 => unsafe { asm!("csrw pmpaddr9, {}", in(reg) addr) },
        10 => unsafe { asm!("csrw pmpaddr10, {}", in(reg) addr) },
        11 => unsafe { asm!("csrw pmpaddr11, {}", in(reg) addr) },
        12 => unsafe { asm!("csrw pmpaddr12, {}", in(reg) addr) },
        13 => unsafe { asm!("csrw pmpaddr13, {}", in(reg) addr) },
        14 => unsafe { asm!("csrw pmpaddr14, {}", in(reg) addr) },
        15 => unsafe { asm!("csrw pmpaddr15, {}", in(reg) addr) },
        _ => return Err(PmpError::InvalidIndex),
    }
    Ok(())
}
