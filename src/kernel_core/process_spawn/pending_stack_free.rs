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

//! Deferred per-process kernel stack release. Teardown runs on the
//! dying capsule's own kernel stack, so the actual `deallocate_page`
//! has to wait until the CPU has context-switched off it. The list
//! is per-CPU; only the originating core drains its own deferred
//! stacks, which keeps the API correct once SMP goes live.
//!
//! The drain runs from the timer trap on whatever kernel stack is
//! current. If `exit_and_yield` is still standing on a deferred stack
//! (e.g. idling because no successor is ready yet), freeing it here is
//! a use-after-free: the frame is reused and zeroed under the live
//! stack, smashing return addresses and heap pointers. So the drain
//! skips any stack the current `rsp` still falls inside and re-queues
//! it for a later tick once the CPU has moved off.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use spin::Mutex;

use crate::arch::{Arch, ArchOps};
use crate::memory::addr::VirtAddr;
use crate::memory::page_allocator::deallocate_page;
use crate::process::core::{Pid, PROCESS_TABLE};
use crate::process::userspace::constants::KERNEL_STACK_SIZE;
use crate::smp::MAX_CPUS;

static PENDING: [Mutex<Vec<u64>>; MAX_CPUS] =
    [const { Mutex::new(Vec::new()) }; MAX_CPUS];

#[inline]
fn slot() -> &'static Mutex<Vec<u64>> {
    let idx = Arch::current_cpu_id() as usize;
    &PENDING[if idx < MAX_CPUS { idx } else { 0 }]
}

pub fn defer_release(pid: Pid) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };
    let top = pcb.kernel_stack_top.swap(0, Ordering::AcqRel);
    if top == 0 {
        return;
    }
    slot().lock().push(top);
}

pub fn drain() {
    // Called from the timer trap. `try_lock` so the trap handler never
    // spins on a `defer_release` caller — they share a CPU and the
    // syscall path holds the lock with interrupts on. A missed tick is
    // harmless; the next tick drains the same entries.
    let drained: Vec<u64> = match slot().try_lock() {
        Some(mut q) => {
            if q.is_empty() {
                return;
            }
            q.drain(..).collect()
        }
        None => return,
    };
    let rsp: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));
    }
    let mut still_live: Vec<u64> = Vec::new();
    for top in drained {
        let base = top - KERNEL_STACK_SIZE as u64;
        if rsp > base && rsp <= top {
            still_live.push(top);
            continue;
        }
        let _ = deallocate_page(VirtAddr::new(base));
    }
    if !still_live.is_empty() {
        if let Some(mut q) = slot().try_lock() {
            q.extend(still_live);
        }
    }
}
