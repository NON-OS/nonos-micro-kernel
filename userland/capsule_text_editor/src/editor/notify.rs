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

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_send;

use super::state::State;

const NDSH_MAGIC: u32 = 0x4E44_5348;
const NDSH_VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const OP_NOTIFY: u16 = 0x0005;
const LEVEL_INFO: u32 = 0;
const NOTIFY_BODY_MAX: usize = 128;
const NOTIFY_REQ_LEN: usize = 8 + NOTIFY_BODY_MAX;

pub(super) fn notify_saved(state: &mut State) {
    if state.shell_port == 0 {
        state.shell_port = lookup_service(b"desktop_shell").map(|p| p.port).unwrap_or(0);
    }
    if state.shell_port == 0 {
        return;
    }
    let mut text = [0u8; 54];
    text[..6].copy_from_slice(b"saved ");
    let n = state.path_len.min(48);
    text[6..6 + n].copy_from_slice(&state.path[..n]);
    send_notify(state.shell_port, LEVEL_INFO, &text[..6 + n]);
}

fn send_notify(port: u32, level: u32, text: &[u8]) {
    let mut frame = [0u8; HDR_LEN + NOTIFY_REQ_LEN];
    frame[0..4].copy_from_slice(&NDSH_MAGIC.to_le_bytes());
    frame[4..6].copy_from_slice(&NDSH_VERSION.to_le_bytes());
    frame[6..8].copy_from_slice(&OP_NOTIFY.to_le_bytes());
    frame[16..20].copy_from_slice(&(NOTIFY_REQ_LEN as u32).to_le_bytes());
    frame[20..24].copy_from_slice(&level.to_le_bytes());
    let n = text.len().min(NOTIFY_BODY_MAX);
    frame[24..28].copy_from_slice(&(n as u32).to_le_bytes());
    frame[28..28 + n].copy_from_slice(&text[..n]);
    let _ = mk_ipc_send(port as u64, frame.as_ptr(), frame.len());
}
