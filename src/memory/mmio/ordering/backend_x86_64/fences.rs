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

use core::sync::atomic::{compiler_fence, Ordering};

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_writes() {
    compiler_fence(Ordering::Release);
    // SAFETY: ek@nonos.systems — the block-level write barrier requires
    // the WC buffer drain that only `sfence` provides; no compiler fence
    // alone would satisfy the contract.
    unsafe {
        core::arch::asm!("sfence", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_reads() {
    // SAFETY: ek@nonos.systems — the block-level read barrier needs
    // hardware load serialisation against WC sources, not just compiler
    // ordering. `lfence` is the right granularity.
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::Acquire);
}

#[inline(always)]
pub(in crate::memory::mmio::ordering) fn fence_full() {
    compiler_fence(Ordering::SeqCst);
    // SAFETY: ek@nonos.systems — `mfence` is the only x86_64 instruction
    // that gives StoreLoad ordering across MMIO and write-back cacheable
    // memory; everything else degrades on mixed mappings.
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
}
