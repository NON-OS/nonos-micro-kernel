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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimerKind {
    TimeWait,
}

#[derive(Clone, Copy)]
struct TimerEntry {
    handle: u32,
    kind: TimerKind,
    deadline_ms: u64,
}

pub struct Timers {
    entries: Vec<TimerEntry>,
}

impl Timers {
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn arm(&mut self, handle: u32, kind: TimerKind, deadline_ms: u64) {
        for e in self.entries.iter_mut() {
            if e.handle == handle && e.kind == kind {
                e.deadline_ms = deadline_ms;
                return;
            }
        }
        self.entries.push(TimerEntry { handle, kind, deadline_ms });
    }

    pub fn cancel_all(&mut self, handle: u32) {
        self.entries.retain(|e| e.handle != handle);
    }

    pub fn next_deadline(&self) -> Option<u64> {
        self.entries.iter().map(|e| e.deadline_ms).min()
    }

    pub fn drain_due(&mut self, now_ms: u64) -> Vec<(u32, TimerKind)> {
        let due = self
            .entries
            .iter()
            .filter(|e| e.deadline_ms <= now_ms)
            .map(|e| (e.handle, e.kind))
            .collect();
        self.entries.retain(|e| e.deadline_ms > now_ms);
        due
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
