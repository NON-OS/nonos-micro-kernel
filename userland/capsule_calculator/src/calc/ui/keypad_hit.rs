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

use super::keypad_geom::stride;
use super::metrics::KEY_GAP;
use crate::calc::buttons::{grid, Role};
use crate::calc::mode::Mode;

pub fn at(mode: Mode, win_w: i32, win_h: i32, x: i32, y: i32) -> Option<(usize, usize)> {
    let (ax, ay, cw, ch, nc, nr) = stride(mode, win_w, win_h);
    if x < ax || y < ay || cw <= 0 || ch <= 0 {
        return None;
    }
    let row = (y - ay) / (ch + KEY_GAP);
    let col = (x - ax) / (cw + KEY_GAP);
    if row >= nr || col >= nc || (y - ay) % (ch + KEY_GAP) >= ch {
        return None;
    }
    let mut start = 0;
    for (idx, btn) in grid(mode).get(row as usize)?.iter().enumerate() {
        let n = btn.span.max(1) as i32;
        if col < start + n {
            let ends = ax + (start + n - 1) * (cw + KEY_GAP) + cw;
            if x >= ends || matches!(btn.role, Role::Blank) {
                return None;
            }
            return Some((row as usize, idx));
        }
        start += n;
    }
    None
}
