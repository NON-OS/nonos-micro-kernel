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

//! A small cyan line-glyph drawn to the left of a menu row: a folder for
//! New Folder, a document for New File. Stroked, not filled, so it matches the
//! line-icon language of the dock.

use crate::render::fill::fill_rect;
use crate::state::Context;

const C: u32 = 0xFF66_E6FF;

pub(super) fn glyph(ctx: &Context, x: u32, y: u32, is_folder: bool) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let stroke = |gx: u32, gy: u32, w: u32, h: u32| fill_rect(va, st, vw, vh, gx, gy, w, h, C);
    if is_folder {
        stroke(x, y + 1, 8, 2); // tab
        stroke(x, y + 3, 20, 1); // top
        stroke(x, y + 15, 20, 1); // bottom
        stroke(x, y + 3, 1, 13); // left
        stroke(x + 19, y + 3, 1, 13); // right
    } else {
        stroke(x + 2, y, 14, 1); // top
        stroke(x + 2, y + 17, 14, 1); // bottom
        stroke(x + 2, y, 1, 18); // left
        stroke(x + 15, y, 1, 18); // right
        stroke(x + 5, y + 6, 8, 1); // text line
        stroke(x + 5, y + 10, 8, 1); // text line
    }
}
