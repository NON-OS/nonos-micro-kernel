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

//! Draw the menu: a rounded, shadowed panel matching the menu-bar drop-downs,
//! with the hovered row lit and one glyph-and-label per item.

use super::glyph::glyph;
use super::height::height;
use super::metrics::{items, label_x, pad_x, pad_y, row_h, width};
use super::origin::origin;
use crate::render::layout::Rect;
use crate::render::palette;
use crate::render::panel::{blend, round_fill, shadow_panel};
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{scale, top_y_centered, UI_PX};
use crate::state::Context;

const GLYPH_H_LOGICAL: u32 = 18;
const INSET_LOGICAL: u32 = 4;
const DELETE_ROW: usize = 2;

pub fn paint(ctx: &Context) {
    if ctx.desktop_menu.is_none() {
        return;
    }
    let (ox, oy) = origin(ctx);
    let (w, rh, s) = (width(ctx), row_h(), scale());
    let inset = INSET_LOGICAL * s;
    let with_glyph = ctx.menu_target.is_none();
    let frame = Rect { x: ox, y: oy, width: w, height: height(ctx) };
    shadow_panel(ctx, frame, palette::R_CARD, palette::PANEL, palette::LINE);

    for (i, label) in items(ctx).iter().enumerate() {
        let top = oy + pad_y() + i as u32 * rh;
        if ctx.menu_hover == Some(i) {
            let width = w.saturating_sub(inset * 2);
            let hl = Rect { x: ox + inset, y: top, width, height: rh };
            round_fill(ctx, hl, palette::R_TILE, palette::ACCENT_HOVER);
        }
        if with_glyph {
            glyph(ctx, ox + pad_x(), top + rh.saturating_sub(GLYPH_H_LOGICAL * s) / 2, i == 0);
        } else {
            let x = ox + pad_x() + 4 * s;
            let tick = Rect { x, y: top + rh / 2 - 3 * s, width: 3 * s, height: 6 * s };
            blend(ctx, tick, palette::ACCENT);
        }
        let danger = !with_glyph && i == DELETE_ROW;
        let fg = if danger { palette::NEGATIVE } else { palette::TEXT };
        text_aa_bytes(ctx, ox + label_x(), top_y_centered(top, rh, UI_PX), label, fg, UI_PX);
    }
}
