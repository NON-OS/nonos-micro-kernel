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

use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::grid::types::Grid;

fn n1(params: &[i64]) -> usize {
    params.first().copied().unwrap_or(1).max(1) as usize
}

fn arg(params: &[i64], i: usize) -> usize {
    params.get(i).copied().unwrap_or(1).max(1) as usize
}

pub fn csi_cursor(g: &mut Grid, c: u8, params: &[i64]) {
    match c {
        b'A' => {
            g.y = g.y.saturating_sub(n1(params));
        }
        b'B' => {
            g.y = (g.y + n1(params)).min(VISIBLE_ROWS - 1);
        }
        b'C' => {
            g.x = (g.x + n1(params)).min(COLS - 1);
        }
        b'D' => {
            g.x = g.x.saturating_sub(n1(params));
        }
        b'E' => {
            g.x = 0;
            g.y = (g.y + n1(params)).min(VISIBLE_ROWS - 1);
        }
        b'F' => {
            g.x = 0;
            g.y = g.y.saturating_sub(n1(params));
        }
        b'G' => {
            g.x = (arg(params, 0) - 1).min(COLS - 1);
        }
        b'H' | b'f' => {
            g.y = (arg(params, 0) - 1).min(VISIBLE_ROWS - 1);
            g.x = (arg(params, 1) - 1).min(COLS - 1);
        }
        b'd' => {
            g.y = (arg(params, 0) - 1).min(VISIBLE_ROWS - 1);
        }
        b'S' => {
            for _ in 0..n1(params) {
                g.scroll_up_one();
            }
        }
        _ => {}
    }
}
