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

//! Row hit-testing for `paint_navlist`, derived from the same `nav_row_h`.

use super::navlist::nav_row_h;

pub(in crate::editor) fn navlist_hit(
    rect: (u32, u32, u32),
    count: usize,
    px: f32,
    mx: i32,
    my: i32,
) -> Option<usize> {
    let (x, y, w) = rect;
    if mx < x as i32 || my < y as i32 || mx >= (x + w) as i32 {
        return None;
    }
    let idx = ((my as u32 - y) / nav_row_h(px)) as usize;
    (idx < count).then_some(idx)
}
