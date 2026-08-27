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

use super::collide;
use super::death;
use super::eat;
use super::pace::pace;
use crate::snake::state::{Game, Phase};

pub fn step(game: &mut Game) -> bool {
    game.advance_clock();
    if game.phase != Phase::Running {
        return false;
    }
    if out_of_time(game) {
        death::finish(game);
        return true;
    }
    pace(game);
    game.dir = game.pending;
    let head = match collide::advance(game) {
        Some(cell) if !collide::blocked(game, cell) => cell,
        _ => return fatal(game),
    };
    game.body.insert(0, head);
    if !eat::consume(game, head) {
        game.body.pop();
    }
    true
}

fn out_of_time(game: &Game) -> bool {
    game.deadline > 0 && game.time_left() == 0
}

// Zen refuses the move rather than ending the run.
fn fatal(game: &mut Game) -> bool {
    if game.mode.is_lethal() {
        death::crash(game);
    }
    true
}
