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

use super::metrics::{NAV_GAP, NAV_H, NAV_TOP, RAIL_PAD_X, RAIL_W};
use crate::calc::mode::{Mode, MODES};

pub fn row_y(i: usize) -> i32 {
    NAV_TOP + (i as i32) * (NAV_H + NAV_GAP)
}

pub fn row_x() -> i32 {
    RAIL_PAD_X
}

pub fn row_w() -> i32 {
    RAIL_W - RAIL_PAD_X * 2
}

pub fn at(x: i32, y: i32) -> Option<Mode> {
    if x < row_x() || x >= row_x() + row_w() || y < NAV_TOP {
        return None;
    }
    let stride = NAV_H + NAV_GAP;
    let i = ((y - NAV_TOP) / stride) as usize;
    if i >= MODES.len() {
        return None;
    }
    if (y - NAV_TOP) % stride >= NAV_H {
        return None;
    }
    Mode::from_index(i)
}
