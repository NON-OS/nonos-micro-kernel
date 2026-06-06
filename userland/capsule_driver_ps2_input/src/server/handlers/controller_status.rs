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
use crate::constants::{
    STATUS_AUX_DATA, STATUS_OFFSET, STATUS_OUTPUT_FULL, STATUS_PARITY, STATUS_TIMEOUT,
};
use crate::poll::read_port;
use crate::protocol::{
    encode_response_header, write_status, Request, CONTROLLER_STATUS_PAYLOAD_LEN, E_IO,
    KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use nonos_libc::mk_ipc_send;
pub fn handle(ctx: &mut Context, req: &Request, tx: &mut [u8]) {
    let status = match read_port(ctx.driver.pio_grant_id, STATUS_OFFSET) {
        Some(v) => v,
        None => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    let payload_len = (STATUS_LEN + CONTROLLER_STATUS_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + STATUS_LEN;
    tx[o] = status;
    tx[o + 1] = ((status & STATUS_OUTPUT_FULL) != 0) as u8;
    tx[o + 2] = ((status & STATUS_PARITY) != 0) as u8;
    tx[o + 3] = ((status & STATUS_TIMEOUT) != 0) as u8;
    o += 4;
    put32(tx, &mut o, ctx.ring.queued() as u32);
    put32(tx, &mut o, ctx.ring.head() as u32);
    put32(tx, &mut o, ctx.ring.tail() as u32);
    put32(tx, &mut o, ((status & STATUS_AUX_DATA) != 0) as u32);
    put32(tx, &mut o, ctx.driver.mouse_enabled as u32);
    put32(tx, &mut o, ctx.mouse_ring.queued() as u32);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}
fn put32(tx: &mut [u8], o: &mut usize, v: u32) {
    tx[*o..*o + 4].copy_from_slice(&v.to_le_bytes());
    *o += 4;
}
