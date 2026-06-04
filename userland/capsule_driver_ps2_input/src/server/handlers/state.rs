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
use crate::protocol::{
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN,
    STATE_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use nonos_libc::mk_ipc_send;
pub fn handle(ctx: &mut Context, req: &Request, tx: &mut [u8]) {
    let payload_len = (STATUS_LEN + STATE_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut off = RESP_HDR_LEN + STATUS_LEN;
    tx[off..off + 8].copy_from_slice(&ctx.ring.events_seen.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.ring.events_dropped.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.ring.parity_errors.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.ring.timeout_errors.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.mouse_ring.events_seen.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.mouse_ring.events_dropped.to_le_bytes());
    off += 8;
    tx[off..off + 8].copy_from_slice(&ctx.mouse_ring.sync_errors.to_le_bytes());
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + (payload_len as usize));
}
