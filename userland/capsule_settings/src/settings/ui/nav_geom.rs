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

use crate::settings::section::{Section, SECTIONS};

use super::metrics::{NAV_GAP, NAV_H, NAV_PAD_X, NAV_TOP, SIDEBAR_W};

pub fn row_y(index: usize) -> u32 {
    NAV_TOP + index as u32 * (NAV_H + NAV_GAP)
}

pub fn row_x() -> u32 {
    NAV_PAD_X / 2
}

pub fn row_w() -> u32 {
    SIDEBAR_W - NAV_PAD_X
}

/// Which nav entry covers `y`, in sidebar-local coordinates. The painter walks
/// the same `row_y`, so a click cannot land on a different entry than the one
/// drawn under the pointer.
pub fn at(x: i32, y: i32) -> Option<Section> {
    if x < 0 || x >= SIDEBAR_W as i32 || y < NAV_TOP as i32 {
        return None;
    }
    let offset = (y - NAV_TOP as i32) as u32;
    let index = (offset / (NAV_H + NAV_GAP)) as usize;
    if offset % (NAV_H + NAV_GAP) >= NAV_H {
        return None;
    }
    SECTIONS.get(index).copied()
}
