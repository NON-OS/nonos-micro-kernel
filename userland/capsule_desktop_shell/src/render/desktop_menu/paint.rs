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

//! Draw the menu: a soft shadow, a rounded panel, and one glyph-and-label row
//! per item with the hovered row lit.

use super::glyph::glyph;
use super::height::height;
use super::metrics::{items, PAD_X, PAD_Y, ROW_H, W};
use super::origin::origin;
use crate::render::fill::fill_rect;
use crate::render::text::draw_overlay_text;
use crate::state::Context;

const SHADOW: u32 = 0xFF07_0B11;
const PANEL: u32 = 0xFF12_1A26;
const BORDER: u32 = 0xFF2E_3A4C;
const HOVER: u32 = 0xFF1E_2C46;
const FG: u32 = 0xFFDF_EAF7;
const GLYPH_H: u32 = 18;

pub fn paint(ctx: &Context) {
    if ctx.desktop_menu.is_none() {
        return;
    }
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let (ox, oy) = origin(ctx);
    let h = height(ctx);
    let rows = items(ctx);
    let with_glyph = ctx.menu_target.is_none();

    // Shadow first, then the panel, with the four corners knocked back to the
    // shadow colour so the edge reads as rounded.
    fill_rect(va, st, vw, vh, ox + 3, oy + 4, W, h, SHADOW);
    fill_rect(va, st, vw, vh, ox, oy, W, h, PANEL);
    for &(cx, cy) in &[(ox, oy), (ox + W - 2, oy), (ox, oy + h - 2), (ox + W - 2, oy + h - 2)] {
        fill_rect(va, st, vw, vh, cx, cy, 2, 2, SHADOW);
    }

    // Border, skipping the knocked corners.
    fill_rect(va, st, vw, vh, ox + 2, oy, W - 4, 1, BORDER);
    fill_rect(va, st, vw, vh, ox + 2, oy + h - 1, W - 4, 1, BORDER);
    fill_rect(va, st, vw, vh, ox, oy + 2, 1, h - 4, BORDER);
    fill_rect(va, st, vw, vh, ox + W - 1, oy + 2, 1, h - 4, BORDER);

    for (i, label) in rows.iter().enumerate() {
        let top = oy + PAD_Y + i as u32 * ROW_H;
        if ctx.menu_hover == Some(i) {
            fill_rect(va, st, vw, vh, ox + 4, top, W - 8, ROW_H, HOVER);
        }
        // The New Folder / New File rows carry a folder or document glyph; the
        // per-item actions are text with a small accent tick.
        if with_glyph {
            glyph(ctx, ox + PAD_X, top + (ROW_H - GLYPH_H) / 2, i == 0);
        } else {
            fill_rect(va, st, vw, vh, ox + PAD_X + 4, top + ROW_H / 2 - 3, 3, 6, 0xFF66_E6FF);
        }
        draw_overlay_text(ctx, ox + PAD_X + 32, top + ROW_H / 2 - 4, label, FG);
    }
}
