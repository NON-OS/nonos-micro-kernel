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
use super::text::draw_status;
use crate::compositor_client::push_damage_commit;
use crate::state::Context;

const TOAST_WIDTH: u32 = 320;
const TOAST_HEIGHT: u32 = 86;
const TOAST_RIGHT_INSET: u32 = 12;
const TOAST_DOCK_GAP: u32 = 10;
const ROW_HEIGHT: u32 = 26;
const ROW_GAP: u32 = 2;
const ACCENT_WIDTH: u32 = 4;
const TEXT_MAX_GLYPHS: usize = 34;
const PANEL_ARGB: u32 = 0xFF0E_1218;
const ROW_ARGB: u32 = 0xFF1B_2030;
const TEXT_ARGB: u32 = 0xFFCF_E6E9;
const TRANSPARENT: u32 = 0x0000_0000;

pub fn toast_rect(display_width: u32, display_height: u32) -> Rect {
    let dock = bottom_dock_rect(display_width, display_height);
    let x = display_width.saturating_sub(TOAST_WIDTH + TOAST_RIGHT_INSET);
    let y = dock.y.saturating_sub(TOAST_HEIGHT + TOAST_DOCK_GAP);
    Rect { x, y, width: TOAST_WIDTH, height: TOAST_HEIGHT }
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

fn paint_toasts(ctx: &Context, r: Rect) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(va, st, w, h, r.x, r.y, r.width, r.height, PANEL_ARGB);
    for (i, toast) in ctx.toasts.iter_live().enumerate() {
        let row_y = r.y + ROW_GAP + i as u32 * (ROW_HEIGHT + ROW_GAP);
        fill_rect(va, st, w, h, r.x + 2, row_y, r.width - 4, ROW_HEIGHT, ROW_ARGB);
        fill_rect(va, st, w, h, r.x + 2, row_y, ACCENT_WIDTH, ROW_HEIGHT, toast.level.tint());
        let glyphs = toast.len.min(TEXT_MAX_GLYPHS);
        draw_status(ctx, r.x + 12, row_y + 9, &toast.text[..glyphs], TEXT_ARGB);
    }
}
