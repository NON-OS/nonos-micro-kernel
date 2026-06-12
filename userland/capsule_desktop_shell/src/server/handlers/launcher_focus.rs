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

use nonos_libc::mk_time_millis;

use crate::render::layout::{bottom_dock_rect, TASKBAR_ENTRY_W};
use crate::server::handlers::launcher_request;
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::{mark_taskbar_launch, Context, LAUNCHER_APPS};

pub fn handle(ctx: &mut Context, x: u32, y: u32) {
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    let mut row_x = bottom.x + 12;
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        if x >= row_x
            && x < row_x + TASKBAR_ENTRY_W
            && y >= bottom.y + 10
            && y < bottom.y + bottom.height - 10
        {
            if launcher_request::request(app) {
                mark_taskbar_launch(&mut ctx.taskbar, index, mk_time_millis());
                refresh_taskbar(ctx);
            }
            return;
        }
        row_x += TASKBAR_ENTRY_W + 6;
    }
}
