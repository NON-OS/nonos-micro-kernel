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
use super::metrics::{items, label_x, pad_x, pad_y, row_h, width};
use super::origin::origin;
use crate::render::fill::fill_rect;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{scale, top_y_centered, UI_PX};
use crate::state::Context;

const SHADOW: u32 = 0xFF07_0B11;
const PANEL: u32 = 0xFF12_1A26;
const BORDER: u32 = 0xFF2E_3A4C;
const HOVER: u32 = 0xFF1E_2C46;
const FG: u32 = 0xFFDF_EAF7;
const GLYPH_H_LOGICAL: u32 = 18;

pub fn paint(ctx: &Context) {
    if ctx.desktop_menu.is_none() {
        return;
    }
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let (ox, oy) = origin(ctx);
    let h = height(ctx);
    let w = width(ctx);
    let rh = row_h();
    let s = scale();
    let rows = items(ctx);
    let with_glyph = ctx.menu_target.is_none();

    // Shadow first, then the panel, with the four corners knocked back to the
    // shadow colour so the edge reads as rounded.
    fill_rect(va, st, vw, vh, ox + 3 * s, oy + 4 * s, w, h, SHADOW);
    fill_rect(va, st, vw, vh, ox, oy, w, h, PANEL);
    let corners =
        [(ox, oy), (ox + w - 2 * s, oy), (ox, oy + h - 2 * s), (ox + w - 2 * s, oy + h - 2 * s)];
    for &(cx, cy) in &corners {
        fill_rect(va, st, vw, vh, cx, cy, 2 * s, 2 * s, SHADOW);
    }

    // Border, skipping the knocked corners.
    fill_rect(va, st, vw, vh, ox + 2 * s, oy, w - 4 * s, s, BORDER);
    fill_rect(va, st, vw, vh, ox + 2 * s, oy + h - s, w - 4 * s, s, BORDER);
    fill_rect(va, st, vw, vh, ox, oy + 2 * s, s, h - 4 * s, BORDER);
    fill_rect(va, st, vw, vh, ox + w - s, oy + 2 * s, s, h - 4 * s, BORDER);

    for (i, label) in rows.iter().enumerate() {
        let top = oy + pad_y() + i as u32 * rh;
        if ctx.menu_hover == Some(i) {
            fill_rect(va, st, vw, vh, ox + 4 * s, top, w - 8 * s, rh, HOVER);
        }
        // The New Folder / New File rows carry a folder or document glyph; the
        // per-item actions are text with a small accent tick.
        if with_glyph {
            glyph(ctx, ox + pad_x(), top + rh.saturating_sub(GLYPH_H_LOGICAL * s) / 2, i == 0);
        } else {
            let tick_x = ox + pad_x() + 4 * s;
            fill_rect(va, st, vw, vh, tick_x, top + rh / 2 - 3 * s, 3 * s, 6 * s, 0xFF66_E6FF);
        }
        text_aa_bytes(ctx, ox + label_x(), top_y_centered(top, rh, UI_PX), label, FG, UI_PX);
    }
}
