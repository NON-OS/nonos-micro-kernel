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

/// Bring the CPU into a usable state on boot. CR0 / CR4 / XCR0 bits
/// for x87, SSE, OSXSAVE, and AVX. Kernel boot path only.
#[cfg(target_arch = "x86_64")]
pub fn init_cpu_features() {
    unsafe {
        let cr0: u64;
        asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack));
        let cr0 = (cr0 | (1 << 1)) & !(1 << 2);
        asm!("mov cr0, {}", in(reg) cr0, options(nomem, nostack));
        let cr4: u64;
        asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
        let cr4 = cr4 | (1 << 9) | (1 << 10) | (1 << 18);
        asm!("mov cr4, {}", in(reg) cr4, options(nomem, nostack));
        let mut xcr0: u64 = 1;
        xcr0 |= 1 << 1;
        xcr0 |= 1 << 2;
        asm!(
            "xsetbv",
            in("ecx") 0u32,
            in("eax") xcr0 as u32,
            in("edx") (xcr0 >> 32) as u32,
            options(nomem, nostack),
        );
    }
}
