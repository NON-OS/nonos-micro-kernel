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

use crate::snake::grid::SPAWN;
use crate::snake::step::{food, walls};

use super::game::Game;
use super::level;
use super::phase::{Dir, Phase};

impl Game {
    // A lost life keeps the score and the level; a fresh run clears both.
    pub fn reset(&mut self, keep_score: bool) {
        if !keep_score {
            self.score = 0;
            self.lives = self.mode.lives();
            self.longest = 0;
            self.elapsed = 0;
            self.deadline = self.mode.time_limit_ms();
        }
        self.level = level::index_for(self.score);
        self.body.clear();
        self.body.push(SPAWN);
        self.body.push((SPAWN.0 - 1, SPAWN.1));
        self.body.push((SPAWN.0 - 2, SPAWN.1));
        self.dir = Dir::Right;
        self.pending = Dir::Right;
        self.phase = Phase::Ready;
        self.streak = 0;
        self.longest = self.longest.max(self.body.len() as u16);
        self.power = None;
        self.slow_until = 0;
        self.base_ms = self.diff.start_ms();
        self.interval_ms = self.base_ms;
        self.last_ms = mk_time_millis();
        self.walls = walls::generate(&mut self.rng, self.level, &self.opts, &self.body);
        self.food = food::place(&mut self.rng, &self.body, &self.walls);
    }
}
