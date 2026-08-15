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

pub enum Step {
    Wait,
    Show(u32),
    Skip(u32),
    End,
}

pub struct Clock {
    start_ms: i64,
    base_pts_ms: i64,
    usec_per_frame: u32,
}

impl Clock {
    pub fn new(start_ms: i64, base_pts_ms: i64, usec_per_frame: u32) -> Clock {
        Clock { start_ms, base_pts_ms, usec_per_frame }
    }

    pub fn pts_ms(&self, frame: u32) -> i64 {
        (frame as i64).saturating_mul(self.usec_per_frame as i64) / 1000
    }

    pub fn due_at(&self, frame: u32) -> i64 {
        self.start_ms
            .saturating_add(self.pts_ms(frame))
            .saturating_sub(self.base_pts_ms)
    }

    pub fn step(&self, next: u32, total: u32, now: i64) -> Step {
        if next >= total {
            return Step::End;
        }
        if now < self.due_at(next) {
            return Step::Wait;
        }
        if next + 1 < total && now >= self.due_at(next + 1) {
            return Step::Skip(next);
        }
        Step::Show(next)
    }
}
