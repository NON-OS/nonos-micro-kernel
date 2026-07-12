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

mod api;
mod balance;
mod constants;
mod interval;
mod percpu_queue;
mod spawn;
mod state;
mod tick;
mod types;

pub use api::*;
pub use balance::try_load_balance;
pub use constants::{DEFAULT_TIME_SLICE, LOAD_BALANCE_INTERVAL_TICKS, MAX_CPUS};
pub use percpu_queue::PerCpuRunQueue;
pub use spawn::{run_local, spawn_on_cpu, spawn_smp};
pub use state::{
    active_cpu_count, for_each_cpu_queue, get_cpu_queue, init_cpu_queue, is_smp_initialized,
};
pub use tick::smp_tick;
pub use types::{CpuLoad, CpuRunQueueStats, LoadBalanceState};
