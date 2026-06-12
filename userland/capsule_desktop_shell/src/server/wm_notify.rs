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

use super::wm_notify_label::resolve_label;
use crate::protocol::{read_u16, read_u32};
use crate::state::{Context, NotifyLevel, TASKBAR_WINDOW_ID};
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
    if event_kind == OPENED {
        let _ = wm_client::window_raise(ctx.wm_port, ctx.issue_request_id(), TASKBAR_WINDOW_ID);
    }
    toast_window_event(ctx, event_kind, owner_pid);
    true
}

fn toast_window_event(ctx: &mut Context, event_kind: u32, owner_pid: u32) {
    let Some(label) = resolve_label(owner_pid) else {
        return;
    };
    let verb: &[u8] = if event_kind == OPENED { b" opened" } else { b" closed" };
    let mut text = [0u8; 48];
    let n = label.len().min(40);
    text[..n].copy_from_slice(&label[..n]);
    text[n..n + verb.len()].copy_from_slice(verb);
    ctx.toasts.push(&text[..n + verb.len()], NotifyLevel::Info, mk_time_millis());
}
