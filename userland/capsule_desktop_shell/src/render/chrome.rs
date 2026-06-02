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
use super::layout::{bottom_dock_rect, menubar_rect, side_dock_rect, spotlight_rect, Rect};
use super::{draw_overlay_text, paint_bottom_taskbar, paint_side_launchers};
use crate::state::Context;

const TRANSPARENT: u32 = 0x0000_0000;
const MENUBAR_ARGB: u32 = 0xFF0E_1218;
const SIDE_DOCK_ARGB: u32 = 0xFF0F_1218;
const BOTTOM_DOCK_ARGB: u32 = 0xFF1B_2030;
const SPOTLIGHT_ARGB: u32 = 0xFF14_1B26;
const TITLE_FG: u32 = 0xFFE1_ECF7;
const CURSOR_ARGB: u32 = 0xFFF7_FBFF;
const CURSOR_SHADOW_ARGB: u32 = 0xFF0A_0D12;
const CURSOR_SIZE: u32 = 12;

pub fn clear_overlay(ctx: &Context) {
    fill_rect(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        0,
        0,
        ctx.width,
        ctx.height,
        TRANSPARENT,
    );
}

pub fn paint_chrome(ctx: &Context) {
    clear_overlay(ctx);
    paint(ctx, menubar_rect(ctx.width), MENUBAR_ARGB);
    paint(ctx, side_dock_rect(ctx.width, ctx.height), SIDE_DOCK_ARGB);
    paint(ctx, bottom_dock_rect(ctx.width, ctx.height), BOTTOM_DOCK_ARGB);
    draw_overlay_text(ctx, 16, 10, b"NONOS launcher", TITLE_FG);
    paint_side_launchers(ctx);
    paint_bottom_taskbar(ctx);
    paint_notify_badge(ctx);
    if ctx.spotlight.visible {
        paint(ctx, spotlight_rect(ctx.width, ctx.height), SPOTLIGHT_ARGB);
    }
    paint_cursor(ctx);
}

const NOTIFY_BADGE: u32 = 8;
const NOTIFY_RIGHT_INSET: u32 = 12;

fn paint_notify_badge(ctx: &Context) {
    let Some(level) = ctx.last_notify_level else { return };
    let bar = menubar_rect(ctx.width);
    if bar.width < NOTIFY_BADGE + NOTIFY_RIGHT_INSET || bar.height < NOTIFY_BADGE + 4 {
        return;
    }
    let x = bar.x + bar.width - NOTIFY_RIGHT_INSET - NOTIFY_BADGE;
    let y = bar.y + (bar.height - NOTIFY_BADGE) / 2;
    paint(ctx, Rect { x, y, width: NOTIFY_BADGE, height: NOTIFY_BADGE }, level.tint());
}

fn paint(ctx: &Context, r: Rect, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, r.x, r.y, r.width, r.height, argb);
}

fn paint_cursor(ctx: &Context) {
    if !ctx.pointer_visible {
        return;
    }
    let x = ctx.pointer_x.min(ctx.width.saturating_sub(CURSOR_SIZE));
    let y = ctx.pointer_y.min(ctx.height.saturating_sub(CURSOR_SIZE));
    paint(ctx, Rect { x: x + 1, y, width: 2, height: CURSOR_SIZE }, CURSOR_SHADOW_ARGB);
    paint(ctx, Rect { x, y: y + 1, width: CURSOR_SIZE, height: 2 }, CURSOR_SHADOW_ARGB);
    paint(ctx, Rect { x, y, width: 2, height: CURSOR_SIZE }, CURSOR_ARGB);
    paint(ctx, Rect { x, y, width: CURSOR_SIZE, height: 2 }, CURSOR_ARGB);
}
