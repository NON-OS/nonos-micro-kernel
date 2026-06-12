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

use super::refresh_taskbar::refresh_taskbar;
use super::wm_notify_app_index::resolve_app_index;
use super::wm_notify_toast::toast_window_event;
use crate::protocol::{read_u16, read_u32};
use crate::state::{set_taskbar_open, Context, TASKBAR_WINDOW_ID};
use crate::wm_client;

const OPENED: u32 = 0;
const CLOSED: u32 = 1;
const MAGIC: u32 = 0x4E57_4D56;
const VERSION: u16 = 1;
const FRAME_LEN: usize = 28;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() != FRAME_LEN || read_u32(buf, 0) != Some(MAGIC) {
        return false;
    }
    if read_u16(buf, 4) != Some(VERSION) {
        return true;
    }
    let (Some(event_kind), Some(owner_pid), Some(window_id)) =
        (read_u32(buf, 8), read_u32(buf, 12), read_u32(buf, 16))
    else {
        return true;
    };
    if event_kind != OPENED && event_kind != CLOSED {
        return true;
    }
    if window_id == TASKBAR_WINDOW_ID {
        return true;
    }
    let opened = event_kind == OPENED;
    if opened {
        let _ = wm_client::window_raise(ctx.wm_port, ctx.issue_request_id(), TASKBAR_WINDOW_ID);
    }
    if let Some(index) = resolve_app_index(owner_pid) {
        set_taskbar_open(&mut ctx.taskbar, index, opened);
        refresh_taskbar(ctx);
    }
    toast_window_event(ctx, opened, owner_pid);
    true
}
