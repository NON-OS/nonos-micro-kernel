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

use crate::arch::context::{setup_initial_user_pcb, SetupError};
use crate::process::core::Pid;

#[derive(Debug, Clone, Copy)]
pub(crate) enum UserEntryError {
    NoSuchProcess,
    NonUserEntry,
    NonUserStack,
}

// Capsule first-run handoff. Delegates to the arch-neutral facade in
// `arch::context::setup`, which classifies the VA against the active
// arch's canonical-user range and writes a per-arch UserEntry into
// `pcb.pending_user_entry`. kernel_sp stays zero — the scheduler
// dispatch hook fills it from `pcb.kernel_stack_top` so there's a
// single source of truth for the per-task kernel sp top.
pub(crate) fn setup_initial_user_context(
    pid: Pid,
    entry: u64,
    user_rsp: u64,
) -> Result<(), UserEntryError> {
    setup_initial_user_pcb(pid, entry, user_rsp).map_err(map_err)
}

fn map_err(e: SetupError) -> UserEntryError {
    match e {
        SetupError::NoSuchProcess => UserEntryError::NoSuchProcess,
        SetupError::NonUserEntry => UserEntryError::NonUserEntry,
        SetupError::NonUserStack => UserEntryError::NonUserStack,
    }
}
