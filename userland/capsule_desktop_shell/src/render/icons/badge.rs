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


use crate::render::fill::blit_rgba8_tinted;
use crate::render::palette;
use crate::render::surface::surface;
use crate::state::Context;

pub const SRC: u32 = 96;

// A real, anti-aliased line-icon (rasterized from SVG) tinted in the app's
// accent, over a translucent rounded tile. One cohesive icon language.
pub fn badge(ctx: &Context, x: u32, y: u32, size: u32, icon: &[u8], accent: u32) {
    {
        let mut buf = surface(ctx);
        let r = (size * 10 / 46).max(2);
        buf.panel(x, y, size, size, r, palette::TILE_FILL, palette::LINE_SOFT);
    }
    glyph(ctx, x, y, size, icon, accent);
}

// The same tinted glyph with no tile behind it, for surfaces that supply their
// own backing (the dock cell) or want none at all (desktop icons).
pub fn glyph(ctx: &Context, x: u32, y: u32, size: u32, icon: &[u8], accent: u32) {
    let g = size * GLYPH_PCT / 100;
    art(ctx, x + size.saturating_sub(g) / 2, y + size.saturating_sub(g) / 2, g, icon, accent);
}

// The art alone, at exactly the size asked for, for callers that already own the
// spacing around it.
pub fn art(ctx: &Context, x: u32, y: u32, size: u32, icon: &[u8], accent: u32) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    blit_rgba8_tinted(va, st, vw, vh, x, y, size, size, icon, SRC, SRC, accent);
}

const GLYPH_PCT: u32 = 46;
