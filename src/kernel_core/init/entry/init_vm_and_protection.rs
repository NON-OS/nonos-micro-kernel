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

//! Paging, then the ring-0 restrictions that only mean something once paging
//! is settled.

use super::fatal::fatal;
use crate::memory::mmu;

pub(super) fn init_vm_and_protection() {
    // VM/paging must be ready before any process creator runs. The process
    // subsystem only initializes its tables after this; the userspace init
    // process itself is created exactly once in `microkernel_main`.
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    crate::sys::bench::mark(b"vm_ready");

    // Here rather than earlier: CR0.WP starts enforcing read-only kernel
    // mappings, so the tables have to be the ones the kernel will keep. SMAP
    // is safe on this design because the kernel reaches user memory only
    // through the directmap, whose leaves the bootloader builds without the
    // user bit, so a supervisor access to a user page never happens. A part
    // without execute-never is fatal: that same directmap is built NX, and
    // with EFER.NXE clear the whole window stays executable.
    if mmu::init_mmu().is_err() {
        fatal("memory: init_mmu failed", "no execute-never support");
    }
    match mmu::protection_flags() {
        Ok(flags) => mmu::report_protection(flags),
        Err(_) => fatal("memory: protection flags unreadable", "mmu not initialised"),
    }
    arm_stack_guards();
}

/// Take the guard page under each of the boot CPU's fault stacks out of the
/// address space. Not fatal on a short count: the machine has been running on
/// these stacks unguarded since GDT setup, so failing to arm leaves it exactly
/// where it already was, and saying so is worth more than halting.
#[cfg(target_arch = "x86_64")]
fn arm_stack_guards() {
    use crate::arch::x86_64::gdt;
    let armed = gdt::arm_bsp_guards();
    let want = gdt::guards_per_cpu();
    crate::sys::serial::print(b"[STACK-GUARD] bsp armed ");
    crate::sys::serial::print_dec(armed as u64);
    crate::sys::serial::print(b"/");
    crate::sys::serial::print_dec(want as u64);
    if armed == want {
        crate::sys::serial::println(b"");
    } else {
        crate::sys::serial::println(b" WARNING overflow can still reach the next stack");
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn arm_stack_guards() {}
