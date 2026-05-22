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

use core::sync::atomic::Ordering;
use x86_64::registers::model_specific::{GsBase, KernelGsBase};
use x86_64::VirtAddr;

use super::types::PerCpuData;
use crate::smp::{cpu_id, MAX_CPUS};

static mut PERCPU_DATA: [PerCpuData; MAX_CPUS] = {
    const INIT: PerCpuData = PerCpuData::new();
    [INIT; MAX_CPUS]
};

// GS_BASE / KERNEL_GS_BASE policy:
//
// Kernel runs with `GS_BASE = &PerCpuData[cpu_id]` so `gs:[off]` from
// kernel code reads the per-CPU record directly. `KERNEL_GS_BASE` is
// the value `swapgs` will swap *into* GS_BASE; we keep it at zero so
// that on every user-mode entry (`swapgs ; iretq` or `swapgs ;
// sysretq`) the user observes a sanitized GS_BASE = 0 and never the
// kernel PerCpuData pointer. On every kernel re-entry from user mode
// the trap/SYSCALL path performs `swapgs`, restoring `GS_BASE` =
// PerCpuData and stashing 0 in `KERNEL_GS_BASE`. Net result:
//   - kernel mode:  GS_BASE = PerCpuData,  KERNEL_GS_BASE = 0
//   - user mode:    GS_BASE = 0,           KERNEL_GS_BASE = PerCpuData
//
// User-side TLS via `gs:` is intentionally unsupported in this slice;
// userland threading uses `fs:` (set per-thread by `arch_prctl`) for
// thread-local storage. A future patch may allow capsule-supplied
// user GS by routing it through a dedicated syscall and writing the
// value into `KERNEL_GS_BASE` while we are in kernel mode (the user-
// visible value lands on the next swapgs).
//
// Every CPL=3 entry/exit path is responsible for the matching swapgs:
//   `jump_to_usermode`, `return_to_usermode_asm`,
//   `restore_user_context_iretq`, `sysret_to_usermode`,
//   `syscall_entry_asm` (entry+exit), and the per-vector trap-entry
//   trampolines (today: `timer_trampoline`).
pub fn init_bsp() {
    // SAFETY: Single-threaded BSP initialization
    unsafe {
        let data = &mut PERCPU_DATA[0];
        data.self_ptr = data as *const PerCpuData as u64;
        data.cpu_id = 0;
        data.apic_id = crate::arch::x86_64::interrupt::apic::id();
        data.random_state.store(read_tsc(), Ordering::Relaxed);

        GsBase::write(VirtAddr::new(data.self_ptr));
        KernelGsBase::write(VirtAddr::new(0));
    }
}

pub fn init_ap(cpu_id: usize) {
    if cpu_id >= MAX_CPUS {
        return;
    }

    // SAFETY: Each AP initializes only its own per-CPU data
    unsafe {
        let data = &mut PERCPU_DATA[cpu_id];
        data.self_ptr = data as *const PerCpuData as u64;
        data.cpu_id = cpu_id as u32;
        data.apic_id = crate::arch::x86_64::interrupt::apic::id();
        data.random_state.store(read_tsc().wrapping_mul(cpu_id as u64 + 1), Ordering::Relaxed);

        GsBase::write(VirtAddr::new(data.self_ptr));
        KernelGsBase::write(VirtAddr::new(0));
    }
}

/// Re-establish a valid per-CPU GS base on trap entry.
///
/// A trap can land in a window where the active GS base is the user
/// value (0); reading per-CPU state through `gs:` then faults on the
/// unmapped zero page and storms the fault handler. When the GS base is
/// not a canonical higher-half kernel pointer, re-derive this CPU's
/// `PerCpuData` from the local APIC id (which needs no GS) and reload
/// the GS base from it before any `gs:`-relative access runs.
pub fn ensure_gs_base() {
    if GsBase::read().as_u64() >= 0xffff_8000_0000_0000 {
        return;
    }
    let apic = crate::arch::x86_64::interrupt::apic::id();
    for i in 0..MAX_CPUS {
        // SAFETY: read-only scan of the initialized per-CPU array.
        let entry = unsafe { &PERCPU_DATA[i] };
        if entry.self_ptr != 0 && entry.apic_id == apic {
            // SAFETY: self_ptr is this CPU's own PerCpuData pointer.
            unsafe { GsBase::write(VirtAddr::new(entry.self_ptr)) };
            return;
        }
    }
    // SAFETY: BSP fallback when the APIC id scan finds no match.
    let bsp = unsafe { &PERCPU_DATA[0] };
    if bsp.self_ptr != 0 {
        unsafe { GsBase::write(VirtAddr::new(bsp.self_ptr)) };
    }
}

#[inline]
pub fn current() -> &'static PerCpuData {
    // SAFETY: cpu_id returns valid index, data is initialized
    unsafe { &PERCPU_DATA[cpu_id()] }
}

#[inline]
pub unsafe fn current_mut() -> &'static mut PerCpuData {
    // SAFETY: Caller ensures exclusive access
    unsafe { &mut PERCPU_DATA[cpu_id()] }
}

pub fn get(id: usize) -> Option<&'static PerCpuData> {
    if id < MAX_CPUS {
        // SAFETY: Index is bounds-checked
        unsafe { Some(&PERCPU_DATA[id]) }
    } else {
        None
    }
}

pub fn set_kernel_stack(stack_top: u64) {
    // SAFETY: Modifying current CPU's data only
    unsafe {
        PERCPU_DATA[cpu_id()].kernel_stack_top = stack_top;
    }
}

#[inline]
pub fn kernel_stack() -> u64 {
    current().kernel_stack_top
}

pub fn set_user_stack(rsp: u64) {
    // SAFETY: Modifying current CPU's data only
    unsafe {
        PERCPU_DATA[cpu_id()].user_stack_saved = rsp;
    }
}

#[inline]
pub fn user_stack() -> u64 {
    current().user_stack_saved
}

pub fn set_current_process(ptr: u64) {
    current().current_process.store(ptr, Ordering::Release);
}

#[inline]
pub fn current_process() -> u64 {
    current().current_process.load(Ordering::Acquire)
}

pub fn set_current_thread(ptr: u64) {
    current().current_thread.store(ptr, Ordering::Release);
}

#[inline]
pub fn current_thread() -> u64 {
    current().current_thread.load(Ordering::Acquire)
}

/// Record the address-space id this CPU has just installed in CR3.
/// `paging::manager::switch_address_space` calls this so the TLB
/// shootdown broadcaster can filter target CPUs by asid.
pub fn set_active_asid(asid: u32) {
    current().active_asid.store(asid, Ordering::Release);
}

#[inline]
pub fn active_asid() -> u32 {
    current().active_asid.load(Ordering::Acquire)
}

#[inline]
pub fn enter_irq() {
    // SAFETY: Modifying current CPU's data only
    unsafe {
        PERCPU_DATA[cpu_id()].irq_nesting += 1;
    }
}

#[inline]
pub fn leave_irq() {
    // SAFETY: Modifying current CPU's data only
    unsafe {
        let data = &mut PERCPU_DATA[cpu_id()];
        if data.irq_nesting > 0 {
            data.irq_nesting -= 1;
        }
    }
}

#[inline]
pub fn in_irq() -> bool {
    current().irq_nesting > 0
}

pub fn percpu_random() -> u64 {
    let data = current();
    let old = data.random_state.load(Ordering::Relaxed);
    let mut x = old;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    data.random_state.store(x, Ordering::Relaxed);
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

#[inline]
fn read_tsc() -> u64 {
    // SAFETY: rdtsc is always safe on x86_64
    unsafe { core::arch::x86_64::_rdtsc() }
}
