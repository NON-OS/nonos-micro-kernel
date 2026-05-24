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

pub fn binary_point() -> u8 {
    let bpr: u64;
    unsafe {
        asm!("mrs {0}, icc_bpr1_el1", out(reg) bpr, options(nostack));
    }
    bpr as u8
}

pub fn set_binary_point(bpr: u8) {
    unsafe {
        asm!("msr icc_bpr1_el1, {0}", in(reg) bpr as u64, options(nostack));
    }
}
