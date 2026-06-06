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
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, MAX_POLL_EVENTS,
    MOUSE_EVENT_WIRE_LEN, MOUSE_POLL_PAYLOAD_PREFIX_LEN, RESP_HDR_LEN,
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
    );
    let _ = mk_irq_ack(ctx.driver.irq_grant_id);
    let _ = mk_irq_ack(ctx.driver.aux_irq_grant_id);
    let mut count: u32 = 0;
    while count < MAX_POLL_EVENTS as u32 {
        let Some(ev) = ctx.mouse_ring.pop() else { break };
        let off =
            RESP_HDR_LEN + MOUSE_POLL_PAYLOAD_PREFIX_LEN + count as usize * MOUSE_EVENT_WIRE_LEN;
        tx[off..off + 2].copy_from_slice(&ev.dx.to_le_bytes());
        tx[off + 2..off + 4].copy_from_slice(&ev.dy.to_le_bytes());
        tx[off + 4] = ev.dz as u8;
        tx[off + 5] = ev.buttons;
        tx[off + 6] = ev.flags;
        tx[off + 7] = 0;
        count += 1;
    }
    let payload_len =
        (MOUSE_POLL_PAYLOAD_PREFIX_LEN + count as usize * MOUSE_EVENT_WIRE_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + 4..RESP_HDR_LEN + 8].copy_from_slice(&count.to_le_bytes());
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}
