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

use core::ptr;

/// Stop the compiler reordering memory operations across this point. Costs no
/// instruction; it constrains the optimizer only, which is what a constant-time
/// sequence needs to keep its shape after inlining.
#[inline(always)]
pub fn compiler_fence() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/// Stop the machine reordering memory operations across this point. Emits the
/// part's full barrier, `mfence` on x86_64 and `dmb ish` on aarch64, so a
/// masked write cannot be observed out of order with the branch that chose it.
#[inline(always)]
pub fn memory_fence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

/// Stop speculation running ahead of this point.
///
/// This is not the same as a memory fence and cannot be written as one: it bounds
/// what the part is allowed to execute speculatively, not what it may reorder
/// architecturally. Each architecture spells that differently, so each is named
/// here rather than approximated by an ordering primitive that would compile to
/// something weaker.
#[inline(always)]
pub fn serialize_execution() {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: LFENCE retires every prior load before any later instruction
    // issues, which on x86_64 also bounds speculation past this point.
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    // SAFETY: ISB flushes the pipeline so nothing fetched after this point was
    // fetched under an earlier prediction. Chosen over `sb`, which is the
    // dedicated speculation barrier but only exists from ARMv8.5, while ISB is
    // baseline and carries the guarantee this call is here for.
    unsafe {
        core::arch::asm!("isb", options(nostack, preserves_flags));
    }
}

#[inline(never)]
pub fn volatile_read<T: Copy>(val: &T) -> T {
    // SAFETY: read_volatile ensures the compiler cannot optimize away this read.
    unsafe { ptr::read_volatile(val) }
}

#[inline(never)]
pub fn volatile_write<T>(dst: &mut T, val: T) {
    // SAFETY: write_volatile ensures the compiler cannot optimize away this write.
    unsafe { ptr::write_volatile(dst, val) };
}

#[inline(never)]
pub fn black_box<T>(val: T) -> T {
    let val = core::hint::black_box(val);
    compiler_fence();
    val
}

#[inline(never)]
pub fn black_box_slice(slice: &[u8]) {
    for byte in slice {
        let _ = volatile_read(byte);
    }
}

#[inline(never)]
pub fn dummy_work(iterations: usize) {
    let mut dummy: u64 = 0;
    for i in 0..iterations {
        dummy = dummy.wrapping_add(i as u64);
        compiler_fence();
    }
    volatile_read(&dummy);
}

#[inline(never)]
pub fn time_constant_execute<F, R>(f: F, dummy_iterations: usize) -> R
where
    F: FnOnce() -> R,
{
    let result = f();
    dummy_work(dummy_iterations);
    result
}
