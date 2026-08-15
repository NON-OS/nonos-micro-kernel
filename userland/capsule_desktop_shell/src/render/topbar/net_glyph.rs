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

//! Three ascending signal bars: cyan when the DHCP lease is bound, dim when the
//! network is down.

use crate::render::fill::fill_rect;
use crate::render::ui_font::scale;
use crate::state::Context;

const ON: u32 = 0xFF66_E6FF;
const OFF: u32 = 0xFF44_505F;

pub(super) fn net_glyph(ctx: &Context, x: u32, y: u32, online: bool) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let color = if online { ON } else { OFF };
    let s = scale();
    let bottom = y + 11 * s;
    for (i, bh) in [4u32, 7, 10].iter().enumerate() {
        let bx = x + i as u32 * 5 * s;
        fill_rect(va, st, w, h, bx, bottom - bh * s, 3 * s, *bh * s, color);
    }
}
