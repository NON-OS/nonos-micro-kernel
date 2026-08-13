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

use super::fill::fill_rect;
use super::layout::{bottom_dock_rect, Rect};
use super::measure_aa::{measure_aa_bytes, truncate_to_width};
use super::text_aa::text_aa;
use super::ui_font::{line_h, top_y_centered, valid_str, UI_PX};
use crate::compositor_client::push_damage_commit;
use crate::state::toasts::MAX_TOASTS;
use crate::state::Context;

const TOAST_MAX_WIDTH: u32 = 460;
const TOAST_MIN_WIDTH: u32 = 260;
const TOAST_RIGHT_INSET: u32 = 12;
const TOAST_DOCK_GAP: u32 = 10;
const ROW_PAD: u32 = 12;
const ROW_GAP: u32 = 2;
const ACCENT_WIDTH: u32 = 4;
const TEXT_INSET: u32 = 12;
const PANEL_ARGB: u32 = 0xFF0E_1218;
const ROW_ARGB: u32 = 0xFF1B_2030;
const TEXT_ARGB: u32 = 0xFFCF_E6E9;
const TRANSPARENT: u32 = 0x0000_0000;

fn row_height() -> u32 {
    line_h(UI_PX) + ROW_PAD
}

fn panel_height() -> u32 {
    ROW_GAP + MAX_TOASTS as u32 * (row_height() + ROW_GAP)
}

fn row_chrome_width() -> u32 {
    4 + ACCENT_WIDTH + TEXT_INSET * 2
}

pub fn toast_rect(display_width: u32, display_height: u32) -> Rect {
    let dock = bottom_dock_rect(display_width, display_height);
    let height = panel_height();
    let width = TOAST_MAX_WIDTH.min(display_width.saturating_sub(TOAST_RIGHT_INSET * 2));
    let x = display_width.saturating_sub(width + TOAST_RIGHT_INSET);
    let y = dock.y.saturating_sub(height + TOAST_DOCK_GAP);
    Rect { x, y, width, height }
}

pub fn sync_toast_layer(ctx: &mut Context) {
    let live = !ctx.toasts.is_empty();
    let r = toast_rect(ctx.width, ctx.height);
    if live {
        paint_toasts(ctx, r);
        ctx.toast_layer_live = true;
    } else {
        if !ctx.toast_layer_live {
            return;
        }
        fill_rect(
            ctx.backing_va,
            ctx.stride,
            ctx.width,
            ctx.height,
            r.x,
            r.y,
            r.width,
            r.height,
            TRANSPARENT,
        );
        ctx.toast_layer_live = false;
    }
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
}

fn panel_width(ctx: &Context, layer_width: u32) -> u32 {
    let widest = ctx
        .toasts
        .iter_live()
        .map(|t| measure_aa_bytes(&t.text[..t.len], UI_PX))
        .max()
        .unwrap_or(0);
    (widest + row_chrome_width()).clamp(TOAST_MIN_WIDTH.min(layer_width), layer_width)
}

fn paint_toasts(ctx: &Context, r: Rect) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(va, st, w, h, r.x, r.y, r.width, r.height, TRANSPARENT);
    let panel_w = panel_width(ctx, r.width);
    let panel_x = r.x + (r.width - panel_w);
    let row_h = row_height();
    fill_rect(va, st, w, h, panel_x, r.y, panel_w, r.height, PANEL_ARGB);
    for (i, toast) in ctx.toasts.iter_live().enumerate() {
        let row_y = r.y + ROW_GAP + i as u32 * (row_h + ROW_GAP);
        fill_rect(va, st, w, h, panel_x + 2, row_y, panel_w.saturating_sub(4), row_h, ROW_ARGB);
        fill_rect(va, st, w, h, panel_x + 2, row_y, ACCENT_WIDTH, row_h, toast.level.tint());
        let text_x = panel_x + 2 + ACCENT_WIDTH + TEXT_INSET;
        let text_max_w = (panel_x + panel_w).saturating_sub(text_x + TEXT_INSET);
        let label = truncate_to_width(valid_str(&toast.text[..toast.len]), UI_PX, text_max_w);
        text_aa(ctx, text_x, top_y_centered(row_y, row_h, UI_PX), label, TEXT_ARGB, UI_PX);
    }
}
