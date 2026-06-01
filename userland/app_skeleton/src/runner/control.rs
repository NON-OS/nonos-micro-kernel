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

use crate::clients::wm;
use crate::discover::lookup_service;

use super::request_id::next;

const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const DESKTOP_SHELL: &[u8] = b"desktop_shell";
const OP_FOCUS_SELF: u16 = 1;

pub(super) fn handle_control(
    buf: &[u8],
    sender_pid: u32,
    wm_port: u32,
    window_id: u32,
    request_id: &mut u32,
) -> bool {
    let Some(op) = control_op(buf) else { return false };
    if !from_desktop_shell(sender_pid) {
        return true;
    }
    if op == OP_FOCUS_SELF {
        let _ = wm::window_raise(wm_port, next(request_id), window_id);
        let _ = wm::window_focus(wm_port, next(request_id), window_id);
    }
    true
}

fn control_op(buf: &[u8]) -> Option<u16> {
    if buf.len() != CONTROL_LEN {
        return None;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let op = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    if magic != CONTROL_MAGIC || version != CONTROL_VERSION {
        return None;
    }
    Some(op)
}

fn from_desktop_shell(sender_pid: u32) -> bool {
    lookup_service(DESKTOP_SHELL)
        .map(|peer| peer.pid == sender_pid)
        .unwrap_or(false)
}
