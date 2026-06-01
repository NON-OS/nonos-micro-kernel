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

use crate::render::layout::{bottom_dock_rect, side_dock_rect, SIDE_ENTRY_H, SIDE_ENTRY_PAD, TASKBAR_ENTRY_W};
use crate::state::{Context, LAUNCHER_APPS};
use crate::server::handlers::launcher_request;

pub fn handle(ctx: &mut Context, x: u32, y: u32) {
    let side = side_dock_rect(ctx.width, ctx.height);
    let mut row_y = side.y + SIDE_ENTRY_PAD;
    for app in LAUNCHER_APPS.iter() {
        if x >= side.x + 6 && x < side.x + side.width - 6 && y >= row_y && y < row_y + SIDE_ENTRY_H {
            launcher_request::request(app);
            return;
        }
        row_y += SIDE_ENTRY_H + SIDE_ENTRY_PAD;
    }
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    let mut row_x = bottom.x + 12;
    for app in LAUNCHER_APPS.iter() {
        if x >= row_x && x < row_x + TASKBAR_ENTRY_W && y >= bottom.y + 10 && y < bottom.y + bottom.height - 10 {
            launcher_request::request(app);
            return;
        }
        row_x += TASKBAR_ENTRY_W + 6;
    }
}
