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

use nonos_libc::{mk_display_vsync_wait, mk_ipc_recv_from};

use crate::discover::lookup_service;

const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const DESKTOP_SHELL: &[u8] = b"desktop_shell";
const OP_FOCUS_SELF: u16 = 1;
const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;

pub(super) fn wait(rx: &mut [u8]) {
    loop {
        let mut sender = 0u32;
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), RECV_BLOCK, &mut sender);
        if n as usize == CONTROL_LEN {
            let frame = &rx[..CONTROL_LEN];
            let magic = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]);
            let version = u16::from_le_bytes([frame[4], frame[5]]);
            let op = u16::from_le_bytes([frame[6], frame[7]]);
            let sender_ok = match lookup_service(DESKTOP_SHELL) {
                Some(peer) => peer.pid == sender,
                None => false,
            };
            if magic == CONTROL_MAGIC && version == CONTROL_VERSION && op == OP_FOCUS_SELF && sender_ok {
                return;
            }
        }
        let _ = mk_display_vsync_wait(0);
    }
}
