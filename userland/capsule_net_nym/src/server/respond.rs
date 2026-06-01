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

pub fn respond(pid: u32, op: u16, errno: u16, request_id: u32, payload_len: u32, tx: &mut [u8]) {
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[8..10].copy_from_slice(&errno.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&payload_len.to_le_bytes());
    if mk_ipc_reply(pid, tx.as_ptr(), HDR_LEN + payload_len as usize) < 0 {
        return;
    }
}
