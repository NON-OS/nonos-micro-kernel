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

//! Block-level MMIO barriers on aarch64.
//!
//! `dsb` rather than `dmb` throughout. `dmb` only orders accesses against each
//! other; `dsb` waits for them to complete, and completion is what a driver
//! means when it says a doorbell must have landed before it looks at a status
//! register. The `sy` domain covers the device end of the interconnect, which
//! the inner-shareable domain does not.

use core::sync::atomic::{compiler_fence, Ordering};

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_writes() {
    compiler_fence(Ordering::Release);
    // SAFETY: `dsb st` takes no operands and touches no register or flag. It
    // waits for every prior store to complete, which is the hardware half of
    // the write barrier this function promises.
    unsafe {
        core::arch::asm!("dsb st", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_reads() {
    // SAFETY: `dsb ld` takes no operands and touches no register or flag. It
    // waits for every prior load to complete before later accesses issue.
    unsafe {
        core::arch::asm!("dsb ld", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::Acquire);
}

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_full() {
    compiler_fence(Ordering::SeqCst);
    // SAFETY: `dsb sy` takes no operands and touches no register or flag. It
    // is the full-system barrier, the only one that orders stores against
    // later loads across both Device and Normal mappings.
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}
