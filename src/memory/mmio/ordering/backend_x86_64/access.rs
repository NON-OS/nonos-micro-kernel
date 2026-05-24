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

/// # Safety
///
/// ek@nonos.systems
///
/// `ptr` must address a kernel-mapped MMIO register exactly
/// `size_of::<T>()` bytes wide, naturally aligned for `T`, on a UC- or
/// WC-attributed page. The function performs one volatile load and
/// returns the bus value verbatim.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn read_relaxed<T: Copy>(ptr: *const T) -> T {
    // SAFETY: ek@nonos.systems — caller has proved the pointer per the fn contract.
    unsafe { core::ptr::read_volatile(ptr) }
}

/// # Safety
///
/// ek@nonos.systems
///
/// `ptr` must address a kernel-mapped MMIO register of width
/// `size_of::<T>()`, naturally aligned, on a UC- or WC-attributed page.
/// The store is the device-visible side effect; whether the value is
/// well-formed for the register is not the function's concern.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn write_relaxed<T: Copy>(ptr: *mut T, value: T) {
    // SAFETY: ek@nonos.systems — caller has proved the pointer per the fn contract.
    unsafe { core::ptr::write_volatile(ptr, value) }
}

/// # Safety
///
/// ek@nonos.systems
///
/// `ptr` is the same MMIO pointer expected by `read_relaxed`. The
/// returned value is observed before any subsequent same-thread access:
/// the `lfence` serialises prior loads at the hardware level (required
/// for WC mappings, where TSO does not), and the compiler fence forbids
/// the optimiser from hoisting later code over the read. The function
/// makes no claim about cross-CPU acquire-release pairing.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn read_acquire<T: Copy>(ptr: *const T) -> T {
    // SAFETY: ek@nonos.systems — single volatile load against the proved pointer.
    let value = unsafe { core::ptr::read_volatile(ptr) };
    // SAFETY: ek@nonos.systems — `lfence` is unconditional on x86_64,
    // takes no operands, and modifies neither registers nor flags.
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }
    compiler_fence(Ordering::Acquire);
    value
}

/// # Safety
///
/// ek@nonos.systems
///
/// `ptr` is the same MMIO pointer expected by `write_relaxed`. The
/// `sfence` ahead of the store drains the write-combining buffer so
/// prior WC writes are observable before this one (required on WC
/// mappings; redundant on UC), and the compiler fence forbids the
/// optimiser from sinking earlier statements past the write. The
/// function makes no claim about cross-CPU acquire-release pairing.
#[inline(always)]
pub(in crate::memory::mmio::ordering) unsafe fn write_release<T: Copy>(ptr: *mut T, value: T) {
    compiler_fence(Ordering::Release);
    // SAFETY: ek@nonos.systems — `sfence` is unconditional on x86_64,
    // takes no operands, and modifies neither registers nor flags.
    unsafe {
        core::arch::asm!("sfence", options(nostack, preserves_flags));
    }
    // SAFETY: ek@nonos.systems — the release envelope is established by
    // the fences above; this is the volatile store itself.
    unsafe { core::ptr::write_volatile(ptr, value) }
}
