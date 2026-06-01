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
use super::layout::{bottom_dock_rect, TASKBAR_ENTRY_W};
use super::text::draw_overlay_text;
use crate::state::{Context, LAUNCHER_APPS};

pub fn paint_bottom_taskbar(ctx: &Context) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let mut x = dock.x + 12;
    for app in LAUNCHER_APPS.iter() {
        fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, x, dock.y + 10, TASKBAR_ENTRY_W, dock.height - 20, 0x2241_5164);
        draw_overlay_text(ctx, x + 8, dock.y + 24, app.icon, 0xFFDA_EAF9);
        draw_overlay_text(ctx, x + 24, dock.y + 24, app.label, 0xFFDA_EAF9);
        x += TASKBAR_ENTRY_W + 6;
    }
}
