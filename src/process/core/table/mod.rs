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

mod build_pcb;
mod claim;
mod create;
mod inherit;
mod ops;
mod pid_alloc;
mod thread_spawn;
mod types;

pub(crate) use create::create_process_with_parent;
pub use claim::{claim_new, release_new};
pub use create::{create_process, create_process_with_mem};
pub use thread_spawn::{admit_thread, spawn_thread, spawn_thread_in, spawn_thread_parked};
pub use types::{allocate_tid, ProcessTable, CURRENT_PID, PROCESS_TABLE};
