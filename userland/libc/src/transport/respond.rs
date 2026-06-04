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

use super::envelope::{HDR_LEN_V2, VERSION_V2};
use crate::ipc::mk_ipc_send;

const E_MSGSIZE: i64 = -90;

pub fn respond(
    magic: u32,
    op: u16,
    errno: u16,
    request_id: u32,
    payload_len: u32,
    request_reply_port: u32,
    default_reply_port: u32,
    tx: &mut [u8],
) -> i64 {
    let total = HDR_LEN_V2 + payload_len as usize;
    if tx.len() < total {
        return E_MSGSIZE;
    }
    tx[0..4].copy_from_slice(&magic.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION_V2.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[8..10].copy_from_slice(&errno.to_le_bytes());
    tx[10..12].copy_from_slice(&0u16.to_le_bytes());
    tx[12..16].copy_from_slice(&0u32.to_le_bytes());
    tx[16..20].copy_from_slice(&request_id.to_le_bytes());
    tx[20..24].copy_from_slice(&payload_len.to_le_bytes());
    let dest =
        if request_reply_port == 0 { default_reply_port as u64 } else { request_reply_port as u64 };
    mk_ipc_send(dest, tx.as_ptr(), total)
}
