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

use nonos_libc::mk_time_millis;

use super::grid::{COLS, ROWS};
use super::rng;

pub const START_INTERVAL_MS: i64 = 160;
pub const MIN_INTERVAL_MS: i64 = 80;
pub const SPEEDUP_MS: i64 = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    pub body: Vec<(i16, i16)>,
    pub dir: Dir,
    pub pending: Dir,
    pub food: (i16, i16),
    pub score: u32,
    pub phase: Phase,
    pub interval_ms: i64,
    pub rng: u64,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            body: Vec::new(),
            dir: Dir::Right,
            pending: Dir::Right,
            food: (0, 0),
            score: 0,
            phase: Phase::Running,
            interval_ms: START_INTERVAL_MS,
            rng: mk_time_millis() as u64 | 1,
        };
        game.reset();
        game
    }

    pub fn reset(&mut self) {
        let center = (COLS / 2, ROWS / 2);
        self.body.clear();
        self.body.push(center);
        self.body.push((center.0 - 1, center.1));
        self.body.push((center.0 - 2, center.1));
        self.dir = Dir::Right;
        self.pending = Dir::Right;
        self.score = 0;
        self.phase = Phase::Running;
        self.interval_ms = START_INTERVAL_MS;
        self.food = rng::place_food(&mut self.rng, &self.body);
    }
}
