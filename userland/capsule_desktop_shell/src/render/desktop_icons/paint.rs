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

//! Draw each desktop entry: its icon and a centred label sitting on a dark pill
//! so the text stays readable over a busy wallpaper.

use super::cell_rect::cell_rect;
use super::metrics::{GLYPH_ADV, ICON, MAX_LABEL};
use crate::render::fill::fill_rect;
use crate::render::icons::draw_fs_icon;
use crate::render::text::draw_overlay_text;
use crate::state::Context;

const LABEL_FG: u32 = 0xFFE4_EEF8;
const PILL_BG: u32 = 0xFF0C_1119;
const EDIT_BG: u32 = 0xFF1E_2C46;
const EDIT_FG: u32 = 0xFFFF_FFFF;
const CARET: u32 = 0xFF66_E6FF;
const PILL_PAD: u32 = 6;
const LABEL_H: u32 = 12;

pub fn paint_desktop_icons(ctx: &Context) {
    for (i, item) in ctx.desktop_items.iter().enumerate() {
        let (cx, cy, cw, _) = cell_rect(ctx, i);
        let icon_x = cx + (cw - ICON) / 2;
        draw_fs_icon(ctx, icon_x, cy, ICON, item.is_dir);

        let label_y = cy + ICON + 8;
        if ctx.rename == Some(i) {
            paint_rename(ctx, cx, cw, label_y);
        } else {
            paint_label(ctx, item.name.as_bytes(), cx, cw, label_y);
        }
    }

    // A dragged icon rides under the cursor until it is dropped.
    if ctx.drag_moved {
        if let Some(item) = ctx.drag_from.and_then(|i| ctx.desktop_items.get(i)) {
            let gx = ctx.drag_x.saturating_sub(ICON / 2);
            let gy = ctx.drag_y.saturating_sub(ICON / 2);
            draw_fs_icon(ctx, gx, gy, ICON, item.is_dir);
        }
    }
}

// The static label on its dark pill.
fn paint_label(ctx: &Context, name: &[u8], cx: u32, cw: u32, label_y: u32) {
    let shown = if name.len() > MAX_LABEL { &name[..MAX_LABEL] } else { name };
    let text_w = shown.len() as u32 * GLYPH_ADV;
    let pill_x = cx + cw.saturating_sub(text_w + PILL_PAD * 2) / 2;
    pill(ctx, pill_x, label_y, text_w, PILL_BG);
    draw_overlay_text(ctx, pill_x + PILL_PAD, label_y, shown, LABEL_FG);
}

// The inline rename field: the edit buffer with a caret, on a lit pill.
fn paint_rename(ctx: &Context, cx: u32, cw: u32, label_y: u32) {
    let text = ctx.rename_buf.as_bytes();
    let shown = if text.len() > MAX_LABEL { &text[text.len() - MAX_LABEL..] } else { text };
    let text_w = shown.len() as u32 * GLYPH_ADV;
    let pill_x = cx + cw.saturating_sub(text_w + PILL_PAD * 2 + GLYPH_ADV) / 2;
    pill(ctx, pill_x, label_y, text_w + GLYPH_ADV, EDIT_BG);
    draw_overlay_text(ctx, pill_x + PILL_PAD, label_y, shown, EDIT_FG);
    let caret_x = pill_x + PILL_PAD + text_w;
    fill_rect(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        caret_x,
        label_y,
        2,
        LABEL_H,
        CARET,
    );
}

fn pill(ctx: &Context, pill_x: u32, label_y: u32, text_w: u32, bg: u32) {
    fill_rect(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        pill_x,
        label_y.saturating_sub(2),
        text_w + PILL_PAD * 2,
        LABEL_H + 4,
        bg,
    );
}
