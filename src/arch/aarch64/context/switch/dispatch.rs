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

use crate::process::core::{ProcessState, PROCESS_TABLE};

use super::first_entry::try_first_entry;
use super::resume::try_resume;

pub(crate) fn switch_to_user_pcb_aarch64(pid: u32) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };

    if try_first_entry(&pcb, pid) {
        return;
    }
    if try_resume(&pcb, pid) {
        return;
    }

    *pcb.state.lock() = ProcessState::Ready;
}
