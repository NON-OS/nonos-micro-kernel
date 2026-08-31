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

use super::{food, walls};
use crate::snake::state::{level, Game};

const FOOD_BASE: u32 = 10;
const FOOD_PER_LEVEL: u32 = 5;

pub fn bite_value(game: &Game) -> u32 {
    let base = FOOD_BASE + game.level as u32 * FOOD_PER_LEVEL;
    if game.power_active() {
        base * 2
    } else {
        base
    }
}

// A power-up halves the pace, so "slow" doubles the step interval.
pub fn pace(game: &mut Game) {
    game.interval_ms = if game.power_active() { game.base_ms * 2 } else { game.base_ms };
}

pub fn speed_up(game: &mut Game) {
    if !game.mode.speeds_up() {
        return;
    }
    game.base_ms = (game.base_ms - game.diff.speedup_ms()).max(game.diff.floor_ms());
}

// Walls regenerate on a level change, clear of the spawn corridor and of the
// body, so levelling up can never drop a block on the snake.
pub fn relevel(game: &mut Game) {
    let reached = level::index_for(game.score);
    if reached == game.level {
        return;
    }
    game.level = reached;
    game.walls = walls::generate(&mut game.rng, game.level, &game.opts, &game.body);
    if game.walls.contains(&game.food) {
        game.food = food::place(&mut game.rng, &game.body, &game.walls);
    }
}
