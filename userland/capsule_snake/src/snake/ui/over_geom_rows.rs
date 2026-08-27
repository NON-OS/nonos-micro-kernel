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

use super::metrics::{GAP_WIDE, ROW_H};
use super::over_geom::body;
use super::rect::{self, Rect};

pub const SUMMARY_ROWS: usize = 6;

pub const HEADS: [&[u8]; SUMMARY_ROWS] =
    [b"Score", b"Length", b"Level", b"Food", b"Time", b"Receipt"];

// The still is the run as it ended, not an illustration, so it keeps the left
// half and the summary reads down the right.
pub fn still(w: u32, h: u32) -> Rect {
    rect::column(body(w, h), 0, 2, GAP_WIDE)
}

pub fn summary(w: u32, h: u32) -> Rect {
    rect::column(body(w, h), 1, 2, GAP_WIDE)
}

pub fn summary_row(w: u32, h: u32, index: usize) -> Rect {
    rect::row(summary(w, h), index.min(SUMMARY_ROWS - 1), ROW_H, 0)
}
