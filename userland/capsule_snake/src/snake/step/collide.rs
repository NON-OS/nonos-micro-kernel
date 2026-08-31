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

use crate::snake::grid::{COLS, ROWS};
use crate::snake::state::Game;

// None means the move left the board with wrapping off: a hard wall hit.
pub fn advance(game: &Game) -> Option<(i16, i16)> {
    let cell = game.dir.step(game.body[0]);
    if in_bounds(cell) {
        return Some(cell);
    }
    if !game.opts.wraps(game.mode) {
        return None;
    }
    Some((cell.0.rem_euclid(COLS), cell.1.rem_euclid(ROWS)))
}

fn in_bounds(cell: (i16, i16)) -> bool {
    cell.0 >= 0 && cell.1 >= 0 && cell.0 < COLS && cell.1 < ROWS
}

// The tail vacates on the same tick the head arrives, so chasing it is legal.
pub fn blocked(game: &Game, cell: (i16, i16)) -> bool {
    let moving = &game.body[..game.body.len() - 1];
    moving.contains(&cell) || game.walls.contains(&cell)
}
