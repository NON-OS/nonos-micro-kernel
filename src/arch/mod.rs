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
mod active_page_table_root;
pub mod console;
pub mod context;
pub mod cpu;
pub mod cpu_random;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub mod fdt;
pub mod halt;
pub(crate) mod idle;
mod init_boot_memory;
mod init_broker_irq_routing;
pub mod interrupt_controller;
pub mod paging;
mod percpu_base;
mod percpu_id;
pub mod port_io;
pub(crate) mod power;
mod remap_pci_windows;
mod run_without_interrupts;
mod stack_pointer;
mod time_counter;
pub mod trap;
pub mod user_access;
pub mod wall_clock;
#[cfg(target_arch = "x86_64")]
pub mod nonos_boot;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
pub use abi::ArchOps;
pub(crate) use active_page_table_root::active_page_table_root;
#[cfg(target_arch = "x86_64")]
pub use cpu::init_cpu_features;
pub use cpu::{cpu_yield, disable_interrupts, enable_interrupts, get_cpu_id, idle_cpu};
pub use halt::halt_loop;
pub(crate) use init_boot_memory::init_boot_memory;
pub(crate) use init_broker_irq_routing::init_broker_irq_routing;
pub(crate) use percpu_base::set as set_percpu_base;
pub(crate) use percpu_base::{cpu_id as percpu_cpu_id, installed as percpu_ready};
pub(crate) use remap_pci_windows::remap_pci_windows;
pub(crate) use run_without_interrupts::run_without_interrupts;
pub(crate) use stack_pointer::stack_pointer;
pub(crate) use time_counter::{read_time_counter, time_counter_hz};
#[cfg(target_arch = "x86_64")]
pub type Arch = x86_64::abi::X86_64;
#[cfg(target_arch = "aarch64")]
pub type Arch = aarch64::abi::Aarch64;
#[cfg(target_arch = "riscv64")]
pub type Arch = riscv64::abi::Riscv64;
#[cfg(target_arch = "x86_64")]
pub use nonos_boot as boot;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
