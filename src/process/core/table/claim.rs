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

//! Taking a process from made to runnable, once.

use super::super::types::{Pid, ProcessState};

/// Move `pid` from `New` to `Ready`. True for the caller that won it;
/// anyone else is looking at a process already started.
pub fn claim_new(pid: Pid) -> bool {
    crate::process::with_process(pid, |pcb| {
        let mut state = pcb.state.lock();
        if !matches!(*state, ProcessState::New) {
            return false;
        }
        *state = ProcessState::Ready;
        true
    })
    .unwrap_or(false)
}

/// Put a claim back, for a caller whose setup failed after winning one.
pub fn release_new(pid: Pid) {
    crate::process::with_process(pid, |pcb| *pcb.state.lock() = ProcessState::New);
}
