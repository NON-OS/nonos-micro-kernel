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
mod init;
mod ipi_dispatch;
mod ipi_handler;
mod preempt;
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
pub use sole_cpu::sole_cpu_apic_id;
pub(crate) use state::{cpu_count, cpus_online};
pub use types::*;
pub fn current_cpu_id() -> u32 {
    cpu_id() as u32
}
#[cfg(target_arch = "x86_64")]
pub use ap::*;
pub use init::*;
pub use ipi_handler::*;
pub use preempt::*;
pub use stats::*;
