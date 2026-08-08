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

use super::super::super::constants::*;
use crate::memory::paging;

pub fn init_module_memory_protection() {
    paging::enable_write_protection();
// SMEP and SMAP are CR4 bits. The same two properties, that the kernel
// cannot execute or read user pages by accident, come from PAN and the
// PXN and UXN table bits on aarch64, and those are set where the tables
// are built rather than from a control register here.
#[cfg(target_arch = "x86_64")]
// SAFETY: reading CR4 has no side effect, and setting SMEP or SMAP only
// tightens what ring 0 may touch.
unsafe {
    let mut cr4: u64;
    core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nostack, preserves_flags));
    if cr4 & CR4_SMEP == 0 {
        cr4 |= CR4_SMEP;
    }
    if cr4 & CR4_SMAP == 0 {
        cr4 |= CR4_SMAP;
    }
    core::arch::asm!("mov cr4, {}", in(reg) cr4, options(nostack, preserves_flags));
}
}
