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

use super::parse_req::HDR_LEN;

pub fn reply(
    sender_pid: u32,
    magic: u32,
    op: u16,
    errno: u16,
    request_id: u32,
    body: &[u8],
    tx: &mut [u8],
) -> i64 {
    tx[0..4].copy_from_slice(&magic.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[8..10].copy_from_slice(&errno.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    tx[HDR_LEN..HDR_LEN + body.len()].copy_from_slice(body);
    mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + body.len())
}
