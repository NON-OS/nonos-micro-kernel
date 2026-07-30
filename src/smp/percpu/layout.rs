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

//! Byte offsets of the `PerCpuData` fields that assembly addresses
//! gs-relative. Every inline asm site takes these as `const` operands
//! instead of hardcoding a number, so a reordered or widened field
//! moves the operand with it instead of silently reading a neighbour.

use super::types::PerCpuData;

pub const SELF_PTR: usize = core::mem::offset_of!(PerCpuData, self_ptr);
pub const CPU_ID: usize = core::mem::offset_of!(PerCpuData, cpu_id);
pub const KERNEL_STACK_TOP: usize = core::mem::offset_of!(PerCpuData, kernel_stack_top);
pub const USER_STACK_SAVED: usize = core::mem::offset_of!(PerCpuData, user_stack_saved);

// The syscall entry stub cannot see Rust constants and repeats two of
// these as `.set PCPU_KERN_STACK` and `.set PCPU_USER_STACK` in
// src/arch/x86_64/asm/syscall.S. These asserts are the tie: a layout
// change fails the build here until the stub is updated to match.
const _: () = assert!(SELF_PTR == 0x00);
const _: () = assert!(CPU_ID == 0x08);
const _: () = assert!(KERNEL_STACK_TOP == 0x20);
const _: () = assert!(USER_STACK_SAVED == 0x28);
