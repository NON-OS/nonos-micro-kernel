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

use crate::process::core::{Pid, PROCESS_TABLE};

use super::enter::SSTATUS_USER_INITIAL;
use super::types::UserEntry;



const USER_VA_MAX_SV39: u64 = (1u64 << 38) - 1;

#[derive(Debug, Clone, Copy)]
pub enum SetupError {
    NoSuchProcess,
    NonUserEntry,
    NonUserStack,
}







pub fn setup_initial_user_pcb_riscv64(
    pid: Pid,
    entry: u64,
    user_sp: u64,
) -> Result<(), SetupError> {
    if entry == 0 || entry > USER_VA_MAX_SV39 {
        return Err(SetupError::NonUserEntry);
    }
    if user_sp == 0 || user_sp > USER_VA_MAX_SV39 {
        return Err(SetupError::NonUserStack);
    }
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or(SetupError::NoSuchProcess)?;
    let entry_ctx = UserEntry {
        entry,
        user_sp,
        sstatus: SSTATUS_USER_INITIAL,
        kernel_sp: 0,
        args: [0; 8],
    };
    *pcb.pending_user_entry.lock() = Some(entry_ctx);
    Ok(())
}
