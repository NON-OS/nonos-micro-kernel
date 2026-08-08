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

//! Cache maintenance by point of coherency, one line at a time.

use crate::memory::addr::VirtAddr;

#[derive(Clone, Copy)]
pub(super) enum Op {
    /// Push what the CPU wrote out to where the device reads.
    Clean,
    /// Drop what the CPU cached, cleaning too: a speculative fill during the
    /// transfer can leave a dirty line that would be written back over the
    /// device's data.
    CleanAndInvalidate,
}

pub(super) fn range(cpu_addr: VirtAddr, size: usize, op: Op) {
    let line = line_bytes();
    let mut addr = cpu_addr.as_u64() & !(line - 1);
    let end = cpu_addr.as_u64().saturating_add(size as u64);
    while addr < end {
        // SAFETY: maintenance by virtual address neither reads nor writes the
        // location, it only moves the line between cache and memory. The
        // range covers a buffer the caller owns.
        unsafe {
            match op {
                Op::Clean => {
                    core::arch::asm!("dc cvac, {}", in(reg) addr, options(nostack, preserves_flags))
                }
                Op::CleanAndInvalidate => {
                    core::arch::asm!("dc civac, {}", in(reg) addr, options(nostack, preserves_flags))
                }
            }
        }
        addr = addr.saturating_add(line);
    }
}

/// Smallest data cache line on this core, from `CTR_EL0.DminLine`, which holds
/// log2 of the size in words. Assuming 64 bytes would skip lines on a core
/// with a smaller one.
fn line_bytes() -> u64 {
    let ctr: u64;
    // SAFETY: CTR_EL0 is readable at EL1 and has no side effects.
    unsafe {
        core::arch::asm!("mrs {}, ctr_el0", out(reg) ctr, options(nomem, nostack, preserves_flags));
    }
    4u64 << ((ctr >> 16) & 0xF)
}

#[inline(always)]
pub(super) fn barrier() {
    // SAFETY: `dsb sy` takes no operands and touches no register or flag.
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}
