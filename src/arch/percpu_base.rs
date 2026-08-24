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

//! The register that tells a CPU where its own per-CPU block is.
//!
//! Every architecture keeps one so an interrupt handler can find its state
//! without consulting a table indexed by a CPU number it would first have to
//! read from somewhere. x86_64 uses `GS_BASE`, with the shadow copy in
//! `KERNEL_GS_BASE` that `swapgs` exchanges on a privilege change; aarch64 uses
//! `TPIDR_EL1`, which EL0 cannot see and so needs no shadow.

/// Point this CPU's per-CPU register at `base`.
///
/// # Safety
///
/// `base` must address a per-CPU block that outlives every use, and must be
/// the block belonging to the calling CPU. Handlers dereference it without
/// further checking, so a wrong value here is read as another CPU's state.
pub(crate) unsafe fn set(base: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        use x86_64::registers::model_specific::{GsBase, KernelGsBase};
        use x86_64::VirtAddr;
        GsBase::write(VirtAddr::new(base));
        // The shadow starts null: nothing has entered from user mode yet, and
        // a stale kernel pointer here would be swapped in by the first
        // `swapgs` on a syscall entry.
        KernelGsBase::write(VirtAddr::new(0));
    }
    #[cfg(target_arch = "aarch64")]
    {
        // SAFETY: TPIDR_EL1 is software-defined scratch, writable at EL1, and
        // invisible to EL0. Writing it has no effect other than the store.
        unsafe {
            core::arch::asm!("msr tpidr_el1, {}", in(reg) base, options(nomem, nostack, preserves_flags));
        }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = base;
}
