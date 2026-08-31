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

use nonos_libc::mk_time_millis;

use super::game::Game;
use super::phase::Phase;

impl Game {
    // Wall-clock deltas rather than a tick count, so a slow frame does not
    // hand the player extra seconds in Time Attack.
    pub fn advance_clock(&mut self) -> i64 {
        let now = mk_time_millis();
        let delta = (now - self.last_ms).max(0);
        self.last_ms = now;
        if self.phase == Phase::Running {
            self.elapsed += delta;
        }
        delta
    }

    pub fn time_left(&self) -> i64 {
        if self.deadline == 0 {
            return 0;
        }
        (self.deadline - self.elapsed).max(0)
    }

    pub fn power_active(&self) -> bool {
        self.slow_until > self.elapsed
    }

    pub fn power_left(&self) -> i64 {
        (self.slow_until - self.elapsed).max(0)
    }
}
