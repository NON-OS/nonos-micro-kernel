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

use crate::render::fill::fill_rect;
use crate::render::layout::Rect;
use crate::state::Context;

pub fn paint_rect(ctx: &Context, r: Rect, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, r.x, r.y, r.width, r.height, argb);
}

pub fn paint_border(ctx: &Context, r: Rect, argb: u32, t: u32) {
    let t = t.min(r.width).min(r.height);
    if t == 0 {
        return;
    }
    let (bw, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(bw, st, w, h, r.x, r.y, r.width, t, argb);
    fill_rect(bw, st, w, h, r.x, r.y + r.height - t, r.width, t, argb);
    fill_rect(bw, st, w, h, r.x, r.y, t, r.height, argb);
    fill_rect(bw, st, w, h, r.x + r.width - t, r.y, t, r.height, argb);
}
