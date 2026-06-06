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

use super::layout::{bottom_dock_rect, menubar_rect, side_dock_rect, spotlight_rect};
use super::{draw_overlay_text, paint_bottom_taskbar, paint_side_launchers};
use crate::state::Context;

mod clear_overlay;
mod constants;
mod paint_cursor;
mod paint_notify_badge;
mod paint_rect;

const MENUBAR_ARGB: u32 = 0xFF0E_1218;
const SIDE_DOCK_ARGB: u32 = 0xFF0F_1218;
const BOTTOM_DOCK_ARGB: u32 = 0xFF1B_2030;
const SPOTLIGHT_ARGB: u32 = 0xFF14_1B26;
const TITLE_FG: u32 = 0xFFE1_ECF7;
const PANEL_BORDER_ARGB: u32 = 0xFF2A_3446;

pub fn paint_chrome(ctx: &Context) {
    clear_overlay::clear_overlay(ctx);
    paint_rect::paint_rect(ctx, menubar_rect(ctx.width), MENUBAR_ARGB);
    paint_rect::paint_rect(ctx, side_dock_rect(ctx.width, ctx.height), SIDE_DOCK_ARGB);
    paint_rect::paint_border(ctx, side_dock_rect(ctx.width, ctx.height), PANEL_BORDER_ARGB, 1);
    paint_rect::paint_rect(ctx, bottom_dock_rect(ctx.width, ctx.height), BOTTOM_DOCK_ARGB);
    paint_rect::paint_border(ctx, bottom_dock_rect(ctx.width, ctx.height), PANEL_BORDER_ARGB, 1);
    draw_overlay_text(ctx, 16, 10, b"NONOS launcher", TITLE_FG);
    paint_side_launchers(ctx);
    paint_bottom_taskbar(ctx);
    paint_notify_badge::paint_notify_badge(ctx);
    if ctx.spotlight.visible {
        paint_rect::paint_rect(ctx, spotlight_rect(ctx.width, ctx.height), SPOTLIGHT_ARGB);
    }
    paint_cursor::paint_cursor(ctx);
}
