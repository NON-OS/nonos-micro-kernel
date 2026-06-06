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

use super::draw_app_icon;
use super::fill::fill_rect;
use super::layout::{bottom_dock_rect, TASKBAR_ENTRY_W};
use crate::state::{Context, LAUNCHER_APPS};

// Bottom-dock glyphs run 22% larger than the side launcher icons.
const ICON_SIZE: u32 = 20;

pub fn paint_bottom_taskbar(ctx: &Context) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let box_top = dock.y + 10;
    let box_h = dock.height - 20;
    let mut x = dock.x + 12;
    for app in LAUNCHER_APPS.iter() {
        fill_rect(
            ctx.backing_va,
            ctx.stride,
            ctx.width,
            ctx.height,
            x,
            box_top,
            TASKBAR_ENTRY_W,
            box_h,
            0x2241_5164,
        );
        let icon_x = x + (TASKBAR_ENTRY_W - ICON_SIZE) / 2;
        let icon_y = box_top + (box_h - ICON_SIZE) / 2;
        draw_app_icon(ctx, icon_x, icon_y, app.icon, ICON_SIZE);
        x += TASKBAR_ENTRY_W + 6;
    }
}
