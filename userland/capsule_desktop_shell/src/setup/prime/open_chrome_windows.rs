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

use crate::render::layout::bottom_dock_rect;
use crate::state::{Context, TASKBAR_WINDOW_ID};
use crate::wm_client;
use nonos_libc::mk_yield;

const WINDOW_KIND_POPUP: u32 = 3;
const OPEN_RETRIES: usize = 16;

// The left side dock duplicated the bottom dock's launcher entries, so it
// is no longer opened. Only the bottom dock chrome window is created;
// launching happens entirely from the bottom dock.
pub fn open_chrome_windows(ctx: &mut Context) -> Result<(), &'static str> {
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    open_retry(ctx, TASKBAR_WINDOW_ID, bottom.x, bottom.y, bottom.width, bottom.height)
        .map_err(|_| "wm rejected taskbar window_open")
}

fn open_retry(
    ctx: &mut Context,
    window_id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(), &'static str> {
    for _ in 0..OPEN_RETRIES {
        if wm_client::window_open(
            ctx.wm_port,
            ctx.issue_request_id(),
            window_id,
            WINDOW_KIND_POPUP,
            x,
            y,
            width,
            height,
        )
        .is_ok()
        {
            return Ok(());
        }
        mk_yield();
    }
    Err("wm rejected window_open")
}
