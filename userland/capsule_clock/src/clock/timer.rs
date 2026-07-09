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

pub struct Timer {
    pub running: bool,
    pub target_ms: u64,
    pub deadline_ms: u64,
    pub remaining_ms: u64,
    pub fired: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            running: false,
            target_ms: 60_000,
            deadline_ms: 0,
            remaining_ms: 60_000,
            fired: false,
        }
    }
}

impl Timer {
    pub fn remaining(&self, now_ms: u64) -> u64 {
        if self.running {
            self.deadline_ms.saturating_sub(now_ms)
        } else {
            self.remaining_ms
        }
    }

    pub fn set(&mut self, target_ms: u64) {
        self.running = false;
        self.target_ms = target_ms;
        self.remaining_ms = target_ms;
        self.fired = false;
    }

    pub fn toggle(&mut self, now_ms: u64) {
        if self.running {
            self.remaining_ms = self.deadline_ms.saturating_sub(now_ms);
            self.running = false;
        } else if self.remaining_ms > 0 {
            self.deadline_ms = now_ms + self.remaining_ms;
            self.running = true;
            self.fired = false;
        }
    }

    pub fn reset(&mut self) {
        self.running = false;
        self.remaining_ms = self.target_ms;
        self.fired = false;
    }

    pub fn poll(&mut self, now_ms: u64) -> bool {
        if self.running && self.deadline_ms <= now_ms {
            self.running = false;
            self.remaining_ms = 0;
            self.fired = true;
            return true;
        }
        false
    }
}
