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

//! Every stack one CPU can be switched onto by a fault, in one block.
//!
//! Held apart from `PerCpuGdt`, whose non-zero descriptor words would drag
//! every stack byte into `.data` with them: 32 MiB of zeros inside the kernel
//! image, hashed by the attestation on every boot. All-zero here, so `.bss`.

use super::constants::MAX_CPUS;
use super::guarded_stack::GuardedStack;

/// Interrupt stacks a TSS can name. The seventh is spare; the IDT gates that
/// use the other six are listed beside the `IST_*` indices.
pub(super) const IST_STACKS: usize = 7;

#[repr(C, align(4096))]
pub struct CpuStacks {
    pub ist: [GuardedStack; IST_STACKS],
    pub kernel: GuardedStack,
}

impl CpuStacks {
    pub const fn new() -> Self {
        const G: GuardedStack = GuardedStack::new();
        Self { ist: [G; IST_STACKS], kernel: G }
    }

    /// Top of IST stack `index`, counted from one to match `TSS.IST[n]`.
    pub fn ist_top(&self, index: usize) -> u64 {
        self.ist[index - 1].top()
    }

    pub fn kernel_top(&self) -> u64 {
        self.kernel.top()
    }
}

pub(crate) static mut BSP_STACKS: CpuStacks = CpuStacks::new();

pub(crate) static mut AP_STACKS: [CpuStacks; MAX_CPUS] = {
    const INIT: CpuStacks = CpuStacks::new();
    [INIT; MAX_CPUS]
};
