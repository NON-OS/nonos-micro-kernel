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

//! Single MMIO register accesses on aarch64.
//!
//! Device-nGnRnE mappings already keep accesses to the same device in program
//! order, but nothing orders them against Normal memory, which is exactly what
//! a descriptor ring needs. So the acquire and release forms carry real
//! barriers here for the same reason they do on x86_64, spelled `dsb` instead
//! of `lfence` and `sfence`.

use core::sync::atomic::{compiler_fence, Ordering};

/// # Safety
///
/// `ptr` must address a kernel-mapped MMIO register exactly
/// `size_of::<T>()` bytes wide and naturally aligned for `T`, on a Device or
/// Normal-non-cacheable mapping. One volatile load is performed and the bus
/// value returned verbatim.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn read_relaxed<T: Copy>(ptr: *const T) -> T {
    // SAFETY: the caller has proved the pointer per the contract above.
    unsafe { core::ptr::read_volatile(ptr) }
}

/// # Safety
///
/// `ptr` must address a kernel-mapped MMIO register of width
/// `size_of::<T>()`, naturally aligned, on a Device or Normal-non-cacheable
/// mapping. The store is the device-visible side effect; whether the value is
/// well formed for the register is not this function's concern.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn write_relaxed<T: Copy>(ptr: *mut T, value: T) {
    // SAFETY: the caller has proved the pointer per the contract above.
    unsafe { core::ptr::write_volatile(ptr, value) }
}

/// # Safety
///
/// `ptr` is the same MMIO pointer `read_relaxed` expects. The returned value
/// is observed before any later same-thread access: the `dsb ld` waits for the
/// load to complete at the device, and the compiler fence stops the optimiser
/// hoisting later code above it. No claim is made about cross-CPU pairing.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn read_acquire<T: Copy>(ptr: *const T) -> T {
    // SAFETY: single volatile load against the proved pointer.
    let value = unsafe { core::ptr::read_volatile(ptr) };
    // SAFETY: `dsb ld` takes no operands and touches no register or flag.
    unsafe {
        core::arch::asm!("dsb ld", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::Acquire);
    value
}

/// # Safety
///
/// `ptr` is the same MMIO pointer `write_relaxed` expects. The `dsb st` ahead
/// of the store waits for earlier stores to complete, so a descriptor written
/// to Normal memory is visible to the device before the doorbell rings, and
/// the compiler fence stops the optimiser sinking earlier statements past it.
/// No claim is made about cross-CPU pairing.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn write_release<T: Copy>(ptr: *mut T, value: T) {
    compiler_fence(Ordering::Release);
    // SAFETY: `dsb st` takes no operands and touches no register or flag.
    unsafe {
        core::arch::asm!("dsb st", options(nostack, preserves_flags));
    }
    // SAFETY: the release envelope is established above; this is the store.
    unsafe { core::ptr::write_volatile(ptr, value) }
}
