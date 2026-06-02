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
use super::draw_app_icon;
use super::layout::{side_dock_rect, SIDE_ENTRY_H, SIDE_ENTRY_PAD};
use super::text::draw_overlay_text;
use crate::state::{Context, LAUNCHER_APPS};

pub fn paint_side_launchers(ctx: &Context) {
    let dock = side_dock_rect(ctx.width, ctx.height);
    let mut y = dock.y + SIDE_ENTRY_PAD;
    for app in LAUNCHER_APPS.iter() {
        fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, dock.x + 6, y, dock.width - 12, SIDE_ENTRY_H, 0x2230_3B4A);
        draw_app_icon(ctx, dock.x + 10, y + 3, app.icon, 16);
        draw_overlay_text(ctx, dock.x + 34, y + 7, app.label, 0xFFD9_E8F8);
        y += SIDE_ENTRY_H + SIDE_ENTRY_PAD;
    }
}
