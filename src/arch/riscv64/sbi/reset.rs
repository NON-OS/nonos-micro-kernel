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

pub fn shutdown() -> ! {
    unsafe {
        asm!("li a7, 0x08", "ecall", options(noreturn));
    }
}

pub fn system_reset(reset_type: u32, reason: u32) -> ! {
    unsafe {
        asm!(
            "li a7, 0x53525354",
            "li a6, 0",
            "mv a0, {0}",
            "mv a1, {1}",
            "ecall",
            in(reg) reset_type,
            in(reg) reason,
            options(noreturn)
        );
    }
}
