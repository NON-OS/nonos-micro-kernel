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
use crate::poll::drain;
use crate::protocol::{
    encode_response_header, write_status, Request, EVENT_WIRE_LEN, KERNEL_REPLY_ENDPOINT,
    MAX_POLL_EVENTS, POLL_PAYLOAD_PREFIX_LEN, RESP_HDR_LEN,
};
use crate::server::context::Context;
use nonos_libc::{mk_ipc_send, mk_irq_ack};
pub fn handle(ctx: &mut Context, req: &Request, tx: &mut [u8]) {
    drain(
        ctx.driver.pio_grant_id,
        &mut ctx.drainer,
        &mut ctx.ring,
        &mut ctx.mouse,
        &mut ctx.mouse_ring,
        ctx.driver.mouse_enabled,
    );
    let _ = mk_irq_ack(ctx.driver.irq_grant_id);
    let _ = mk_irq_ack(ctx.driver.aux_irq_grant_id);
    let mut count: u32 = 0;
    let cap = MAX_POLL_EVENTS as u32;
    let body_off = RESP_HDR_LEN + POLL_PAYLOAD_PREFIX_LEN;
    while count < cap {
        let ev = match ctx.ring.pop() {
            Some(ev) => ev,
            None => break,
        };
        let off = body_off + (count as usize) * EVENT_WIRE_LEN;
        tx[off] = ev.scancode;
        tx[off + 1] = ev.flags;
        tx[off + 2] = 0;
        count += 1;
    }
    let payload_len = (POLL_PAYLOAD_PREFIX_LEN + (count as usize) * EVENT_WIRE_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + 4..RESP_HDR_LEN + 8].copy_from_slice(&count.to_le_bytes());
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + (payload_len as usize));
}
