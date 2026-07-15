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

// cpu_yield: halt until next interrupt with the current mask state.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn cpu_yield() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[inline(always)]
pub fn cpu_yield() {
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}
