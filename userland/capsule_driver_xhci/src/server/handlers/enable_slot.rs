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
use nonos_libc::mk_ipc_send;
use crate::controller::{issue_disable_slot, issue_enable_slot};
use crate::protocol::{
    encode_response_header, write_status, Request, E_IO, E_NODEV, KERNEL_REPLY_ENDPOINT,
    RESP_HDR_LEN, SLOT_ENABLE_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, tx: &mut [u8]) {
    let slot_id = match issue_enable_slot(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
    ) {
        Ok(slot_id) => slot_id,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    if !ctx.driver.slots.mark_allocated(slot_id, ctx.driver.layout.max_slots) {
        let _ = issue_disable_slot(
            ctx.driver.layout.doorbell_base,
            ctx.driver.layout.primary_intr_base,
            &mut ctx.driver.command_ring,
            &mut ctx.driver.event_ring,
            slot_id,
        );
        reply_with_status(tx, req, E_NODEV);
        return;
    }
    let payload_len = (STATUS_LEN + SLOT_ENABLE_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN] = slot_id;
    tx[RESP_HDR_LEN + STATUS_LEN + 1..RESP_HDR_LEN + STATUS_LEN + 4].fill(0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}