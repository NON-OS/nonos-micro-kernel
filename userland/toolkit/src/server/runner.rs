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

extern crate alloc;

use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_ipc_send_to_pid};

use crate::animation;
use crate::component_dispatch;
use crate::protocol::{
    decode, encode, Header, E_BAD_OP, E_SHORT, HDR_LEN, IPC_PAYLOAD_MAX, STATUS_OK,
    THEME_PAYLOAD_LEN, TOOLKIT_ENDPOINT, TOOLKIT_OP_ANIMATION_TICK, TOOLKIT_OP_COMPONENT_RENDER,
    TOOLKIT_OP_HEALTHCHECK, TOOLKIT_OP_THEME_APPLY, TOOLKIT_OP_THEME_GET,
};
use crate::theme;

const ENOTSUP: i64 = -95;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(TOOLKIT_ENDPOINT, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n == ENOTSUP {
            mk_exit(0);
        }
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let used = n as usize;
        let Some(hdr) = decode(&rx[..used]) else {
            continue;
        };
        let payload = &rx[HDR_LEN..used];
        let (status, reply_len) = dispatch(hdr.op, payload, &mut tx[HDR_LEN..]);
        let reply_hdr =
            Header { op: hdr.op, request_id: hdr.request_id, payload_len: reply_len as u32 };
        encode(&mut tx[..HDR_LEN], &reply_hdr, status);
        let _ = mk_ipc_send_to_pid(sender_pid, tx.as_ptr(), HDR_LEN + reply_len);
    }
}

fn dispatch(op: u16, payload: &[u8], reply: &mut [u8]) -> (u16, usize) {
    match op {
        TOOLKIT_OP_HEALTHCHECK => (STATUS_OK, 0),
        TOOLKIT_OP_THEME_APPLY => (theme::apply(payload), 0),
        TOOLKIT_OP_THEME_GET => theme_get(reply),
        TOOLKIT_OP_ANIMATION_TICK => animation::tick(payload, reply),
        TOOLKIT_OP_COMPONENT_RENDER => (component_dispatch::render(payload), 0),
        _ => {
            let _ = E_SHORT;
            (E_BAD_OP, 0)
        }
    }
}

fn theme_get(reply: &mut [u8]) -> (u16, usize) {
    if reply.len() < THEME_PAYLOAD_LEN {
        return (E_BAD_OP, 0);
    }
    let snap = theme::snapshot();
    reply[0..4].copy_from_slice(&snap.background_argb.to_le_bytes());
    reply[4..8].copy_from_slice(&snap.surface_argb.to_le_bytes());
    reply[8..12].copy_from_slice(&snap.accent_argb.to_le_bytes());
    reply[12..16].copy_from_slice(&snap.text_argb.to_le_bytes());
    reply[16..20].copy_from_slice(&snap.border_argb.to_le_bytes());
    reply[20..24].copy_from_slice(&snap.revision.to_le_bytes());
    (STATUS_OK, THEME_PAYLOAD_LEN)
}
