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

//! Wiping the per-process kernel stacks at shutdown. They come from the page
//! allocator, not the heap, and sit in the kernel half, so neither the heap
//! erase nor the process wipe reaches them; they hold user bytes a syscall
//! copied in. The running stack is skipped by address, not pid, since the wipe
//! runs on whichever stack the caller entered on.

use super::erase::secure_zero;
use crate::memory::addr::VirtAddr;
use crate::memory::layout;
use crate::memory::paging::manager;
use crate::process::userspace::constants::KERNEL_STACK_SIZE;
use core::sync::atomic::Ordering;

/// Zero every process kernel stack except the one in use. Returns the number
/// of pages wiped.
pub(super) fn wipe_kernel_stacks() -> usize {
    let page = layout::PAGE_SIZE as u64;
    let here = crate::arch::stack_pointer();
    let mut wiped = 0;

    for process in crate::process::enumerate_all_processes() {
        let Some(pcb) = crate::process::get_process_table().find_by_pid(process.pid) else {
            continue;
        };
        let top = pcb.kernel_stack_top.load(Ordering::Acquire);
        if top == 0 {
            continue;
        }
        let Some(base) = top.checked_sub(KERNEL_STACK_SIZE as u64) else {
            continue;
        };
        // The live stack. Erasing it would take the return addresses this call
        // still needs with it.
        if here >= base && here < top {
            continue;
        }

        let mut va = base;
        while va < top {
            // Kernel stack pages are mapped in the kernel half, which every
            // address space shares, so translating in the current one finds
            // them. Going through the directmap keeps this identical to the
            // user-range wipe and skips anything already unmapped.
            if let Some(pa) = manager::translate_address(VirtAddr::new(va)) {
                let direct = layout::DIRECTMAP_BASE.wrapping_add(pa.as_u64());
                // SAFETY: the translation says this frame is mapped, and the
                // directmap covers physical memory read-write for the kernel.
                // The stack is not the running one and the other CPUs are
                // stopped, so nothing is executing on it.
                secure_zero(direct as *mut u8, page as usize);
                wiped += 1;
            }
            va = match va.checked_add(page) {
                Some(next) => next,
                None => break,
            };
        }
    }
    wiped
}
