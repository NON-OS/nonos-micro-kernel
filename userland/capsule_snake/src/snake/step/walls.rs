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

use crate::snake::grid::{COLS, ROWS, SPAWN, SPAWN_CLEAR_X, SPAWN_CLEAR_Y};
use crate::snake::rng;
use crate::snake::state::{level, Options};

const TRIES: u32 = 512;

pub fn generate(
    state: &mut u64,
    level_idx: usize,
    opts: &Options,
    body: &[(i16, i16)],
) -> Vec<(i16, i16)> {
    let mut cells = Vec::new();
    if !opts.obstacles {
        return cells;
    }
    let target = level::walls(level_idx);
    let mut tries = 0;
    while cells.len() < target && tries < TRIES {
        tries += 1;
        let cell = rng::cell(state, COLS, ROWS);
        if near_spawn(cell) || body.contains(&cell) || cells.contains(&cell) {
            continue;
        }
        cells.push(cell);
    }
    cells
}

// The corridor the snake spawns into. Keeping walls out of it is what makes a
// level change survivable; the body test above covers a change mid-run.
pub fn near_spawn(cell: (i16, i16)) -> bool {
    (cell.0 - SPAWN.0).abs() <= SPAWN_CLEAR_X && (cell.1 - SPAWN.1).abs() <= SPAWN_CLEAR_Y
}
