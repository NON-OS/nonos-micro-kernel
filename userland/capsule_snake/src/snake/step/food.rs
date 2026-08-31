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
use crate::snake::rng;

const TRIES: u32 = 64;
const POWER_EVERY: u32 = 5;

pub fn place(state: &mut u64, body: &[(i16, i16)], walls: &[(i16, i16)]) -> (i16, i16) {
    for _ in 0..TRIES {
        let cell = rng::cell(state, COLS, ROWS);
        if free(cell, body, walls) {
            return cell;
        }
    }
    first_free(body, walls)
}

// A drop every fifth bite, never on the body, a wall, or the food itself.
pub fn drop_power(
    state: &mut u64,
    streak: u32,
    body: &[(i16, i16)],
    walls: &[(i16, i16)],
    food: (i16, i16),
) -> Option<(i16, i16)> {
    if streak == 0 || streak % POWER_EVERY != 0 {
        return None;
    }
    for _ in 0..TRIES {
        let cell = rng::cell(state, COLS, ROWS);
        if free(cell, body, walls) && cell != food {
            return Some(cell);
        }
    }
    None
}

fn free(cell: (i16, i16), body: &[(i16, i16)], walls: &[(i16, i16)]) -> bool {
    !body.contains(&cell) && !walls.contains(&cell)
}

fn first_free(body: &[(i16, i16)], walls: &[(i16, i16)]) -> (i16, i16) {
    for y in 0..ROWS {
        for x in 0..COLS {
            if free((x, y), body, walls) {
                return (x, y);
            }
        }
    }
    (0, 0)
}
