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

use super::constants::{ICC_SRE_DFB, ICC_SRE_DIB, ICC_SRE_ENABLE, ICC_SRE_SRE};
use super::priority::set_priority_mask;

pub fn init() {
    enable_system_register_access();
    set_priority_mask(0xFF);
    enable_group1();
}

fn enable_system_register_access() {
    let sre = ICC_SRE_SRE | ICC_SRE_DFB | ICC_SRE_DIB | ICC_SRE_ENABLE;
    unsafe {
        asm!("msr icc_sre_el1, {0}", "isb", in(reg) sre, options(nostack));
    }
}

fn enable_group1() {
    unsafe {
        asm!("msr icc_igrpen1_el1, {0}", "isb", in(reg) 1u64, options(nostack));
    }
}
