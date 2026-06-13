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

pub fn next(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

pub fn place_food(state: &mut u64, body: &[(i16, i16)]) -> (i16, i16) {
    for _ in 0..64 {
        let r = next(state);
        let cell = ((r % COLS as u64) as i16, ((r >> 16) % ROWS as u64) as i16);
        if !body.contains(&cell) {
            return cell;
        }
    }
    first_free(body)
}

fn first_free(body: &[(i16, i16)]) -> (i16, i16) {
    for y in 0..ROWS {
        for x in 0..COLS {
            if !body.contains(&(x, y)) {
                return (x, y);
            }
        }
    }
    (0, 0)
}
