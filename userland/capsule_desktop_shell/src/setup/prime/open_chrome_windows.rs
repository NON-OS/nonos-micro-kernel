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

use crate::render::layout::{bottom_dock_rect, side_dock_rect};
use crate::state::{Context, SIDE_DOCK_WINDOW_ID, TASKBAR_WINDOW_ID};
use crate::wm_client;
use nonos_libc::mk_yield;

const WINDOW_KIND_POPUP: u32 = 3;
const OPEN_RETRIES: usize = 16;

pub fn open_chrome_windows(ctx: &mut Context) -> Result<(), &'static str> {
    let side = side_dock_rect(ctx.width, ctx.height);
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    open_retry(ctx, SIDE_DOCK_WINDOW_ID, side.x, side.y, side.width, side.height)?;
    if open_retry(ctx, TASKBAR_WINDOW_ID, bottom.x, bottom.y, bottom.width, bottom.height).is_err()
    {
        let _ = wm_client::window_close(ctx.wm_port, ctx.issue_request_id(), SIDE_DOCK_WINDOW_ID);
        return Err("wm rejected taskbar window_open");
    }
    Ok(())
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
