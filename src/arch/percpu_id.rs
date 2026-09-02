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

//! This CPU's number, read out of its own per-CPU block.
//!
//! One load, and it cannot name the wrong CPU: the value comes from the block
//! the per-CPU register points at, written by this CPU with its own block.
//! Every syscall asks for it, because the current process is tracked per CPU
//! and capability checks are keyed on it.
//!
//! Reading it dereferences that register, so it cannot also decide whether the
//! register is valid. `installed` answers that, and one global flag suffices:
//! the boot CPU is alone until `smp::init_bsp` runs, so while the flag is false
//! there is one CPU and its number is 0; an AP comes up with the flag already
//! true and installs its own base first, which `ap_entry` does and says so.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::smp::percpu::layout::CPU_ID;

static INSTALLED: AtomicBool = AtomicBool::new(false);

#[inline]
pub(crate) fn installed() -> bool {
    INSTALLED.load(Ordering::Relaxed)
}

/// Called by `percpu_base::set` once the register is live, never before.
#[inline]
pub(super) fn mark_installed() {
    INSTALLED.store(true, Ordering::Release);
}

/// # Safety
/// The per-CPU register must already point at this CPU's block, which the
/// caller establishes by checking `installed()`.
#[inline]
pub(crate) unsafe fn cpu_id() -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let id: u32;
        // SAFETY: ek@nonos.systems - the caller promised GS_BASE addresses
        // this CPU's `PerCpuData`, and `CPU_ID` is that struct's own field
        // offset, asserted against the layout the entry stubs use.
        unsafe {
            core::arch::asm!(
                "mov {0:e}, gs:[{1}]",
                out(reg) id,
                const CPU_ID,
                options(nostack, preserves_flags, readonly),
            );
        }
        id
    }
    #[cfg(not(target_arch = "x86_64"))]
    // SAFETY: ek@nonos.systems - as above, through TPIDR_EL1.
    unsafe {
        let base: u64;
        core::arch::asm!("mrs {}, tpidr_el1", out(reg) base, options(nomem, nostack));
        core::ptr::read_volatile((base as usize + CPU_ID) as *const u32)
    }
}
