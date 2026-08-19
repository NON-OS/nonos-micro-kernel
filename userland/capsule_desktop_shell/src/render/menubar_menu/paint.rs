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

//! Draw the titles across the menu bar and, when one is open, its floating
//! drop-down over everything the desktop already painted.

use super::items::{rows, title, TITLE_COUNT};
use super::metrics::{inset, pad_x, pad_y, panel_h, panel_w, row_h, row_pad_x, title_w, title_x};
use super::origin::origin;
use crate::render::layout::{menubar_height, Rect};
use crate::render::palette;
use crate::render::panel::{round_fill, shadow_panel};
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{top_y_centered, UI_PX};
use crate::state::Context;

pub fn paint_titles(ctx: &Context) {
    let bar_h = menubar_height();
    for i in 0..TITLE_COUNT {
        let x = title_x(ctx, i);
        let w = title_w(ctx, i);
        if x + w > ctx.width {
            break;
        }
        let open = ctx.menubar.open == Some(i);
        if open {
            let box_h = bar_h.saturating_sub(inset() * 2);
            let r = Rect { x, y: inset(), width: w, height: box_h };
            round_fill(ctx, r, palette::R_TILE, palette::ACCENT_DIM);
        }
        let fg = if open { palette::TEXT } else { palette::TEXT_DIM };
        let top = top_y_centered(0, bar_h, UI_PX);
        text_aa(ctx, x + pad_x(), top, title(ctx, i), fg, UI_PX);
    }
}

pub fn paint(ctx: &Context) {
    let Some(index) = ctx.menubar.open else {
        return;
    };
    let (ox, oy) = origin(ctx, index);
    let w = panel_w(ctx, index);
    let r = Rect { x: ox, y: oy, width: w, height: panel_h(ctx, index) };
    shadow_panel(ctx, r, palette::R_CARD, palette::PANEL, palette::LINE);
    for (i, label) in rows(ctx, index).iter().enumerate() {
        let top = oy + pad_y() + i as u32 * row_h();
        if ctx.menubar.hover == Some(i) {
            let width = w.saturating_sub(inset() * 2);
            let hl = Rect { x: ox + inset(), y: top, width, height: row_h() };
            round_fill(ctx, hl, palette::R_TILE, palette::ACCENT_HOVER);
        }
        let text_y = top_y_centered(top, row_h(), UI_PX);
        text_aa(ctx, ox + row_pad_x(), text_y, label, palette::TEXT, UI_PX);
    }
}
