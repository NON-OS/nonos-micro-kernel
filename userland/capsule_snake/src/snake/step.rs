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

use super::grid::{COLS, ROWS};
use super::rng;
use super::state::{Dir, Game, Phase, MIN_INTERVAL_MS, SPEEDUP_MS};

pub fn step(game: &mut Game) -> bool {
    if game.phase != Phase::Running {
        return false;
    }
    game.dir = game.pending;
    let (hx, hy) = game.body[0];
    let next = match game.dir {
        Dir::Up => (hx, hy - 1),
        Dir::Down => (hx, hy + 1),
        Dir::Left => (hx - 1, hy),
        Dir::Right => (hx + 1, hy),
    };
    let hits_wall = next.0 < 0 || next.1 < 0 || next.0 >= COLS || next.1 >= ROWS;
    let moving_body = &game.body[..game.body.len() - 1];
    if hits_wall || moving_body.contains(&next) {
        game.phase = Phase::GameOver;
        return true;
    }
    game.body.insert(0, next);
    if next == game.food {
        game.score += 1;
        game.interval_ms = (game.interval_ms - SPEEDUP_MS).max(MIN_INTERVAL_MS);
        game.food = rng::place_food(&mut game.rng, &game.body);
    } else {
        game.body.pop();
    }
    true
}
