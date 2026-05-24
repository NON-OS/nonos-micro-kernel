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

pub fn console_putchar(c: u8) {
    unsafe {
        asm!("li a7, 0x01", "mv a0, {0}", "ecall", in(reg) c as usize, options(nostack));
    }
}

pub fn console_getchar() -> Option<u8> {
    let ret: isize;
    unsafe {
        asm!("li a7, 0x02", "ecall", "mv {0}, a0", out(reg) ret, options(nostack));
    }
    if ret >= 0 {
        Some(ret as u8)
    } else {
        None
    }
}
