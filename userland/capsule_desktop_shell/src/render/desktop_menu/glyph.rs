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
use crate::render::palette;
use crate::render::ui_font::scale;
use crate::state::Context;

const C: u32 = palette::ACCENT;

pub(super) fn glyph(ctx: &Context, x: u32, y: u32, is_folder: bool) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let stroke = |gx: u32, gy: u32, w: u32, h: u32| fill_rect(va, st, vw, vh, gx, gy, w, h, C);
    let s = scale();
    if is_folder {
        stroke(x, y + s, 8 * s, 2 * s); // tab
        stroke(x, y + 3 * s, 20 * s, s); // top
        stroke(x, y + 15 * s, 20 * s, s); // bottom
        stroke(x, y + 3 * s, s, 13 * s); // left
        stroke(x + 19 * s, y + 3 * s, s, 13 * s); // right
    } else {
        stroke(x + 2 * s, y, 14 * s, s); // top
        stroke(x + 2 * s, y + 17 * s, 14 * s, s); // bottom
        stroke(x + 2 * s, y, s, 18 * s); // left
        stroke(x + 15 * s, y, s, 18 * s); // right
        stroke(x + 5 * s, y + 6 * s, 8 * s, s); // text line
        stroke(x + 5 * s, y + 10 * s, 8 * s, s); // text line
    }
}
