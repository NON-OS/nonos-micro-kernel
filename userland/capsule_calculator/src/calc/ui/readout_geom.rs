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

use super::metrics::{PANE_PAD, RAIL_W, READOUT_H};

pub fn origin() -> (i32, i32) {
    (RAIL_W + PANE_PAD, PANE_PAD)
}

pub fn size(win_w: i32) -> (i32, i32) {
    (win_w - RAIL_W - PANE_PAD * 2, READOUT_H)
}

pub fn inset() -> i32 {
    PANE_PAD - 4
}
