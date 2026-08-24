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

//! The fault and kernel stacks, held apart from the descriptors that point at
//! them.
//!
//! These used to be inline arrays inside `PerCpuGdt`. A GDT has non-zero
//! descriptor words, so the whole struct was a non-zero initialiser and the
//! linker put it in `.data` — stacks included. At eight 16 KiB stacks per CPU
//! and 256 CPUs that is 32 MiB of zeros carried inside the kernel image, hashed
//! by the attestation on every boot and copied off the ESP before anything
//! runs. Split out, the stacks are an all-zero initialiser and land in `.bss`,
//! which occupies no bytes in the image at all.
//!
//! Keeping them in one place per CPU also means the guard pages these have
//! never had can be installed around a known, page-aligned span rather than
//! around fields embedded in a descriptor table.

use super::constants::{DEFAULT_STACK_SIZE, MAX_CPUS};

/// Interrupt stacks a TSS can name. The seventh is spare; the IDT gates that
/// use the other six are listed beside the `IST_*` indices.
pub const IST_STACKS: usize = 7;

#[repr(C, align(4096))]
pub struct CpuStacks {
    pub ist: [[u8; DEFAULT_STACK_SIZE]; IST_STACKS],
    pub kernel: [u8; DEFAULT_STACK_SIZE],
}

impl CpuStacks {
    pub const fn new() -> Self {
        Self { ist: [[0; DEFAULT_STACK_SIZE]; IST_STACKS], kernel: [0; DEFAULT_STACK_SIZE] }
    }

    /// Top of IST stack `index`, counted from one to match `TSS.IST[n]`.
    pub fn ist_top(&self, index: usize) -> u64 {
        self.ist[index - 1].as_ptr() as u64 + DEFAULT_STACK_SIZE as u64
    }

    pub fn kernel_top(&self) -> u64 {
        self.kernel.as_ptr() as u64 + DEFAULT_STACK_SIZE as u64
    }
}

pub(crate) static mut BSP_STACKS: CpuStacks = CpuStacks::new();

pub(crate) static mut AP_STACKS: [CpuStacks; MAX_CPUS] = {
    const INIT: CpuStacks = CpuStacks::new();
    [INIT; MAX_CPUS]
};
