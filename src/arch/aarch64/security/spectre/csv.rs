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

pub fn is_csv2_enabled() -> bool {
    ((read_aa64pfr0() >> 56) & 0xF) >= 1
}

pub fn is_csv3_enabled() -> bool {
    ((read_aa64pfr0() >> 60) & 0xF) >= 1
}

fn read_aa64pfr0() -> u64 {
    let aa64pfr0: u64;
    unsafe {
        asm!("mrs {}, id_aa64pfr0_el1", out(reg) aa64pfr0);
    }
    aa64pfr0
}
