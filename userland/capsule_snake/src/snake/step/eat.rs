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

use super::food;
use super::pace::{bite_value, pace, relevel, speed_up};
use crate::snake::state::Game;

const POWER_BONUS: u32 = 50;
const POWER_MS: i64 = 6_000;

// True when the snake grew, so the caller knows to keep the tail.
pub fn consume(game: &mut Game, head: (i16, i16)) -> bool {
    if game.power == Some(head) {
        take_power(game);
        return false;
    }
    if head != game.food {
        return false;
    }
    take_food(game);
    true
}

fn take_power(game: &mut Game) {
    game.power = None;
    game.slow_until = game.elapsed + POWER_MS;
    game.score += POWER_BONUS;
    relevel(game);
    pace(game);
}

fn take_food(game: &mut Game) {
    game.score += bite_value(game);
    game.streak += 1;
    game.longest = game.longest.max(game.body.len() as u16);
    game.food = food::place(&mut game.rng, &game.body, &game.walls);
    if game.opts.powerups && game.power.is_none() {
        game.power =
            food::drop_power(&mut game.rng, game.streak, &game.body, &game.walls, game.food);
    }
    speed_up(game);
    relevel(game);
    pace(game);
}
