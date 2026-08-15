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

use crate::render::fill::{blit_rgba8_tinted, fill_rect};
use crate::render::ui_font;
use crate::state::Context;

const TILE_BG: u32 = 0xFF0C1016;
const SRC: u32 = 48; // icon assets are 48x48 RGBA

// A real, anti-aliased line-icon (rasterized from SVG) tinted in the app's
// accent, centered on a rounded tile. One cohesive, production icon language.
pub fn badge(ctx: &Context, x: u32, y: u32, size: u32, icon: &[u8], accent: u32) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(va, st, vw, vh, x, y, size, size, TILE_BG);

    // rounded corners: knock a 2px stair off each corner
    let bg = 0xFF0B1019;
    let sc = ui_font::scale();
    let s1 = size.saturating_sub(sc);
    let s2 = size.saturating_sub(2 * sc);
    for &(cx, cy) in &[(x, y), (x + s2, y), (x, y + s2), (x + s2, y + s2)] {
        fill_rect(va, st, vw, vh, cx, cy, 2 * sc, 2 * sc, bg);
    }
    // a thin, dim accent frame skipping the corners
    let f = dim(accent, 45);
    let span = s2.saturating_sub(2 * sc);
    fill_rect(va, st, vw, vh, x + 2 * sc, y, span, sc, f);
    fill_rect(va, st, vw, vh, x + 2 * sc, y + s1, span, sc, f);
    fill_rect(va, st, vw, vh, x, y + 2 * sc, sc, span, f);
    fill_rect(va, st, vw, vh, x + s1, y + 2 * sc, sc, span, f);

    // centered tinted glyph, ~60% of the tile
    let g = size * 60 / 100;
    let gx = x + size.saturating_sub(g) / 2;
    let gy = y + size.saturating_sub(g) / 2;
    blit_rgba8_tinted(va, st, vw, vh, gx, gy, g, g, icon, SRC, SRC, accent);
}

// Scale a colour toward black by `pct` percent, preserving alpha.
fn dim(argb: u32, pct: u32) -> u32 {
    let r = ((argb >> 16) & 0xFF) * pct / 100;
    let g = ((argb >> 8) & 0xFF) * pct / 100;
    let b = (argb & 0xFF) * pct / 100;
    0xFF00_0000 | (r << 16) | (g << 8) | b
}
