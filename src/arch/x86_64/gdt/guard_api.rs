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

//! Arming the stack guards, once for the boot CPU and once per application
//! processor. Both run after the paging manager is up, which is the only
//! precondition; the stacks themselves have been in use since GDT setup.
//!
//! Each returns the number of guards it took out of the address space so the
//! caller can compare it against `guards_per_cpu` and say on the console
//! whether this CPU ended up fully guarded.

use core::ptr::addr_of;

use super::constants::MAX_CPUS;
use super::percpu_guards::{arm, GUARDS_PER_CPU};
use super::percpu_stacks::{AP_STACKS, BSP_STACKS};

pub fn arm_bsp_guards() -> usize {
    // SAFETY: ek@nonos.systems - `BSP_STACKS` is a static that lives for the
    // whole run and belongs to this CPU alone. The shared reference is used
    // only to read guard addresses; the stacks in use are reached through RSP
    // and the TSS, never through this path.
    arm(unsafe { &*addr_of!(BSP_STACKS) })
}

/// `cpu_id` is the slot the BSP handed this CPU, so the block is its own.
pub fn arm_ap_guards(cpu_id: u32) -> usize {
    let idx = cpu_id as usize;
    if idx == 0 || idx > MAX_CPUS {
        return 0;
    }
    // SAFETY: ek@nonos.systems - slot `idx - 1` was handed to this CPU alone
    // and is never reused, so nothing else reads or writes this block.
    arm(unsafe { &(*addr_of!(AP_STACKS))[idx - 1] })
}

/// How many guard pages one CPU expects to arm.
pub const fn guards_per_cpu() -> usize {
    GUARDS_PER_CPU
}
