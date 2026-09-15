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

extern crate alloc;

#[cfg(target_arch = "x86_64")]
mod ap;
mod constants;
mod cpu;
mod cpu_id;
mod init;
mod ipi_dispatch;
mod ipi_handler;
mod preempt;
mod responsive;
mod sole_cpu;
mod state;
mod stats;
mod types;

pub mod ipi;
pub mod percpu;
pub mod topology;
#[cfg(target_arch = "x86_64")]
pub mod trampoline;

pub use constants::*;
pub use cpu::*;
pub use cpu_id::cpu_id;
pub use sole_cpu::sole_cpu_apic_id;
pub(crate) use state::{cpu_count, cpu_is_online, cpus_online};
/// Called by the IDT builder, which is the only point early enough that every
/// CPU is guaranteed to load a table carrying these vectors.
#[cfg(target_arch = "x86_64")]
pub(crate) use ipi_dispatch::install_gates as install_ipi_gates;
pub use types::*;
pub fn current_cpu_id() -> u32 {
    cpu_id() as u32
}
#[cfg(target_arch = "x86_64")]
pub use ap::*;
pub use init::*;
pub use ipi_handler::*;
pub use preempt::*;
pub use responsive::lock_responsive;
pub use stats::*;
