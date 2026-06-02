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

// Selector truth, mirrored from `arch::x86_64::gdt::constants`:
//   GDT offset 0x08  kernel_code   → KERNEL_CS = 0x08
//   GDT offset 0x10  kernel_data   → KERNEL_DS = 0x10
//   GDT offset 0x18  user_data     → USER_DS  = 0x18 | 3 = 0x1B
//   GDT offset 0x20  user_code_64  → USER_CS  = 0x20 | 3 = 0x23
// The order in the GDT (data before code) matches the layout STAR
// expects for SYSRET — SYSRET CS = base + 16, SS = base + 8.
pub const USER_CS: u16 = 0x23;
pub const USER_DS: u16 = 0x1B;
pub const KERNEL_CS: u16 = 0x08;
pub const KERNEL_DS: u16 = 0x10;

pub const USER_RFLAGS: u64 = 0x202;
pub const USER_STACK_SIZE: usize = 2 * 1024 * 1024;
pub const KERNEL_STACK_SIZE: usize = 32 * 1024;
pub const USER_STACK_BASE: u64 = 0x0000_7FFF_FFFF_0000;
pub const USER_HEAP_START: u64 = 0x0000_0001_0000_0000;
pub const USER_CODE_START: u64 = 0x0000_0000_0040_0000;
