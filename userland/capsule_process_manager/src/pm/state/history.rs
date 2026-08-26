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

use alloc::vec::Vec;

use super::samples::Ring;

// Whole-system history plus one ring per live pid. Rings for pids that leave
// the table are dropped on the next refresh, so the vector tracks the live set
// and never grows without bound.
pub struct History {
    pub total: Ring,
    per_pid: Vec<(u32, Ring)>,
}

impl History {
    pub fn new() -> Self {
        History { total: Ring::new(), per_pid: Vec::new() }
    }

    pub fn get(&self, pid: u32) -> Option<&Ring> {
        self.per_pid.iter().find(|(p, _)| *p == pid).map(|(_, r)| r)
    }

    pub fn record(&mut self, pid: u32, cpu: u8, mem_kb: u64) {
        match self.per_pid.iter_mut().find(|(p, _)| *p == pid) {
            Some((_, ring)) => ring.push(cpu, mem_kb),
            None => {
                let mut ring = Ring::new();
                ring.push(cpu, mem_kb);
                self.per_pid.push((pid, ring));
            }
        }
    }

    pub fn retain_live(&mut self, live: &[u32]) {
        self.per_pid.retain(|(pid, _)| live.contains(pid));
    }
}
