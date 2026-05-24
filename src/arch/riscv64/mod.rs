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

pub mod abi;
pub mod asm;
pub mod boot;
mod constants;
pub mod context;
pub mod cpu;
pub mod fpu;
pub mod interrupts;
pub mod mmu;
pub mod plic;
pub mod sbi;
pub mod security;
pub mod timer;
pub mod uart;

pub use abi::Riscv64;

pub use boot::init;
pub use constants::{PAGE_SIZE, STACK_SIZE};
pub use cpu::{cpu_id, disable_interrupts, enable_interrupts, halt};
pub use interrupts::TrapFrame;
pub use mmu::{init_mmu, map_page, unmap_page, PageTable};
pub use plic::{enable_irq, init_plic, Plic};
pub use sbi::{console_getchar, console_putchar, set_timer, shutdown};
pub use timer::{current_time_ns, init_timer, read_time};
pub use uart::{init_uart, putc, puts};
