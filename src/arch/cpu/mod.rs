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

//! Free-function shims that route through the active arch backend.
//!
//! Generic kernel code can call any of the names below. Internally
//! every shim dispatches through `<Arch as ArchOps>::method()` so
//! the implementation stays one place per arch.
//!
//! `init_cpu_features` is x86_64-only kernel-startup setup of CR0
//! / CR4 / XCR0 bits. It does not belong in the cross-arch trait
//! and stays here as a cfg-gated free function.

mod cpu_yield;
mod disable_interrupts;
mod enable_interrupts;
mod get_cpu_id;
mod idle_cpu;
#[cfg(target_arch = "x86_64")]
mod init_cpu_features;
mod interrupts_enabled;
mod read_time_counter;

pub use cpu_yield::cpu_yield;
pub use disable_interrupts::disable_interrupts;
pub use enable_interrupts::enable_interrupts;
pub use get_cpu_id::get_cpu_id;
pub use idle_cpu::idle_cpu;
#[cfg(target_arch = "x86_64")]
pub use init_cpu_features::init_cpu_features;
pub use interrupts_enabled::interrupts_enabled;
pub use read_time_counter::read_time_counter;
