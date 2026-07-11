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
use crate::state::Context;

/// Paint one rect of a glyph designed on a 16-unit grid, scaled to `size`
/// pixels. The same art renders at any icon size, so the bottom dock can run
/// larger than the side launcher without redrawing the glyphs.
pub fn paint_u(
    ctx: &Context,
    x0: u32,
    y0: u32,
    size: u32,
    dx: u32,
    dy: u32,
    dw: u32,
    dh: u32,
    argb: u32,
) {
    let px = x0 + dx * size / 16;
    let py = y0 + dy * size / 16;
    // Single-unit dimensions are strokes (lines and outlines). Render them one
    // grid-unit wider than the naive scale so the linework reads bolder at every
    // icon size, the way a heavier pen weight would. Wider spans (fills) keep
    // their exact geometry.
    let stroke = core::cmp::max(2, 2 * size / 16);
    let pw = if dw == 1 { stroke } else { core::cmp::max(1, dw * size / 16) };
    let ph = if dh == 1 { stroke } else { core::cmp::max(1, dh * size / 16) };
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, px, py, pw, ph, argb);
}
