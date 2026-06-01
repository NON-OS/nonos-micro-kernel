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

use crate::state::{Context, SIDE_DOCK_WINDOW_ID, TASKBAR_WINDOW_ID};
use crate::wm_client;

const OPENED: u32 = 0;
const MAGIC: u32 = 0x4E57_4D56;
const VERSION: u16 = 1;
const FRAME_LEN: usize = 28;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() != FRAME_LEN || u32::from_le_bytes(buf[0..4].try_into().unwrap()) != MAGIC {
        return false;
    }
    if u16::from_le_bytes(buf[4..6].try_into().unwrap()) != VERSION {
        return true;
    }
    if u32::from_le_bytes(buf[8..12].try_into().unwrap()) != OPENED {
        return true;
    }
    let window_id = u32::from_le_bytes(buf[16..20].try_into().unwrap());
    if window_id == SIDE_DOCK_WINDOW_ID || window_id == TASKBAR_WINDOW_ID {
        return true;
    }
    let _ = wm_client::window_raise(ctx.wm_port, ctx.issue_request_id(), SIDE_DOCK_WINDOW_ID);
    let _ = wm_client::window_raise(ctx.wm_port, ctx.issue_request_id(), TASKBAR_WINDOW_ID);
    true
}
