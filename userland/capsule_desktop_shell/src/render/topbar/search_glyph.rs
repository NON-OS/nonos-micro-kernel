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

//! The magnifier in the status cluster: a ring and a stub handle, drawn on the
//! shared paint layer so it scales with everything else.

use crate::render::surface::surface;
use crate::render::ui_font::scale;
use crate::state::Context;

use super::super::palette;

const FG: u32 = palette::TEXT_DIM;

pub(super) fn search_glyph(ctx: &Context, x: u32, y: u32) {
    let s = scale();
    let r = (5 * s) as i32;
    let (cx, cy) = ((x + 5 * s) as i32, (y + 5 * s) as i32);
    let reach = r * 7 / 10;
    let mut fb = surface(ctx);

    fb.ring(cx as u32, cy as u32, r as u32, s.max(1), FG);
    for k in 0..s.max(1) as i32 {
        fb.line_aa(cx + reach + k, cy + reach, cx + r + 4 * s as i32 + k, cy + r + 4 * s as i32, FG);
    }
}
