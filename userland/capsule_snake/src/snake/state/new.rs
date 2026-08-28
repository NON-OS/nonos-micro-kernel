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

use super::difficulty::Difficulty;
use super::game::Game;
use super::mode::Mode;
use super::options::Options;
use super::phase::{Dir, Phase};
use super::screen::Screen;

impl Game {
    pub fn new() -> Self {
        let diff = Difficulty::Normal;
        let mode = Mode::Arcade;
        let mut game = Game {
            screen: Screen::Home,
            phase: Phase::Ready,
            mode,
            diff,
            opts: Options::new(),
            body: Vec::new(),
            walls: Vec::new(),
            dir: Dir::Right,
            pending: Dir::Right,
            food: (0, 0),
            power: None,
            score: 0,
            lives: mode.lives(),
            level: 0,
            streak: 0,
            longest: 0,
            base_ms: diff.start_ms(),
            interval_ms: diff.start_ms(),
            elapsed: 0,
            deadline: 0,
            slow_until: 0,
            last_ms: mk_time_millis(),
            runs: Vec::new(),
            awards: Vec::new(),
            rng: mk_time_millis() as u64 | 1,
        };
        game.reset(false);
        crate::snake::store::load_into(&mut game);
        game
    }
}
