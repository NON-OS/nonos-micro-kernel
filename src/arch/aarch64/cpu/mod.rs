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

mod barrier;
mod control;
pub mod features;
mod halt;
pub mod id;
mod init;
mod irq;
pub mod state;
mod wait;

pub use barrier::{data_sync_barrier, instruction_barrier, memory_barrier};
pub use features::{has_feature, CpuFeature};
pub use halt::halt;
pub use id::{cluster_id, core_id, cpu_affinity, cpu_id, mpidr_affinity, pack_affinity};
pub use init::init_cpu;
pub use irq::{disable_interrupts, enable_interrupts, interrupts_enabled};
pub use state::{current_el, is_el1, is_el2};
pub use wait::{send_event, wait_for_event};
