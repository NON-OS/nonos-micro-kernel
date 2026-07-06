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

#[derive(Default)]
pub struct Stopwatch {
    pub running: bool,
    pub start_ms: u64,
    pub accumulated_ms: u64,
}

impl Stopwatch {
    pub fn elapsed(&self, now_ms: u64) -> u64 {
        self.accumulated_ms
            + if self.running {
                now_ms.saturating_sub(self.start_ms)
            } else {
                0
            }
    }

    pub fn toggle(&mut self, now_ms: u64) {
        if self.running {
            self.accumulated_ms += now_ms.saturating_sub(self.start_ms);
            self.running = false;
        } else {
            self.start_ms = now_ms;
            self.running = true;
        }
    }

    pub fn reset(&mut self) {
        self.running = false;
        self.start_ms = 0;
        self.accumulated_ms = 0;
    }
}
