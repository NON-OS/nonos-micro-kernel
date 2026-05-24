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

use core::sync::atomic::Ordering;

use super::types::CapsuleState;
use crate::process::{get_process, ProcessState};

impl CapsuleState {
    pub fn set_alive(&self, pid: u32) {
        self.pid.store(pid, Ordering::SeqCst);
        self.generation.fetch_add(1, Ordering::SeqCst);
    }
    pub fn mark_dead(&self) {
        self.pid.store(0, Ordering::SeqCst);
    }
    pub fn pid(&self) -> u32 {
        self.pid.load(Ordering::SeqCst)
    }
    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }
    pub fn is_alive(&self) -> bool {
        let pid = self.pid.load(Ordering::SeqCst);
        if pid == 0 {
            return false;
        }
        match get_process(pid) {
            Some(pcb) => {
                let alive = matches!(
                    *pcb.state.lock(),
                    ProcessState::New
                        | ProcessState::Ready
                        | ProcessState::Running
                        | ProcessState::Sleeping
                        | ProcessState::Stopped
                );
                if !alive {
                    self.mark_dead();
                }
                alive
            }
            None => {
                self.mark_dead();
                false
            }
        }
    }
}
