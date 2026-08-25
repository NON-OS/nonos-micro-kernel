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

//! Taking the guard pages out of the address space.
//!
//! Until this runs, a stack that overflows keeps writing into whatever the
//! linker put below it, which for a `CpuStacks` block is the next stack down:
//! a deep #PF would quietly eat the #NMI stack and the machine would carry on
//! with two stacks interleaved. Unmapping the guard turns that into a fault at
//! the first byte past the end.
//!
//! It cannot happen at GDT setup, which runs long before the paging manager
//! exists, so the stacks live mapped for the early part of boot and the guards
//! are armed as soon as there is something able to unmap them. The kernel
//! image is mapped a page at a time by the bootloader, so removing one 4 KiB
//! page needs no table split.
//!
//! A CPU arms its own block. The kernel half is shared, so arming is visible
//! everywhere at once, but keeping it per-CPU means only the blocks belonging
//! to CPUs that actually exist are ever touched.

use super::percpu_stacks::{CpuStacks, IST_STACKS};
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::api::unmap_page;

/// Unmap the guard page under every stack in `stacks`.
///
/// Reports how many were taken out. A page that cannot be unmapped is left
/// mapped rather than treated as armed: the count is what the caller prints,
/// and a short count is the honest signal that this block is not fully
/// guarded.
pub(super) fn arm(stacks: &CpuStacks) -> usize {
    let mut armed = 0;
    for slot in &stacks.ist {
        if unmap_page(VirtAddr::new(slot.guard_base())).is_ok() {
            armed += 1;
        }
    }
    if unmap_page(VirtAddr::new(stacks.kernel.guard_base())).is_ok() {
        armed += 1;
    }
    armed
}

/// Every stack a `CpuStacks` block carries, for callers checking the count.
pub(super) const GUARDS_PER_CPU: usize = IST_STACKS + 1;
