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

use nonos_libc::mk_ipc_reply;

use crate::protocol::MAGIC;

const HDR_LEN: usize = 20;
const EMSGSIZE: i64 = -90;

pub fn write_header(
    out: &mut [u8],
    op: u16,
    errno: u16,
    request_id: u32,
    payload_len: u32,
) -> Option<usize> {
    if out.len() < HDR_LEN {
        return None;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&1u16.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&errno.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
    Some(HDR_LEN + payload_len as usize)
}

pub fn respond(
    sender_pid: u32,
    op: u16,
    errno: u16,
    request_id: u32,
    payload_len: u32,
    tx: &mut [u8],
) -> i64 {
    let Some(total) = write_header(tx, op, errno, request_id, payload_len) else {
        return EMSGSIZE;
    };
    if total > tx.len() {
        return EMSGSIZE;
    }
    mk_ipc_reply(sender_pid, tx.as_ptr(), total)
}
