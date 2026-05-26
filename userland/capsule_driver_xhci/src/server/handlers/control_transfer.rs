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
use crate::controller::issue_control_transfer;
use crate::dma::DmaRegion;
use crate::protocol::{
    encode_response_header, write_status, Request,
    CONTROL_TRANSFER_REPLY_PREFIX, CONTROL_TRANSFER_REQUEST_LEN,
    E_INVAL, E_IO, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONTROL_TRANSFER_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let (slot, bm_rt, b_req) = (body[0], body[2], body[3]);
    let w_value = u16::from_le_bytes([body[4], body[5]]);
    let w_index = u16::from_le_bytes([body[6], body[7]]);
    let w_len = u16::from_le_bytes([body[8], body[9]]);
    let region = if w_len > 0 {
        match ctx.driver.dma_pool.alloc(w_len as u64) {
            Ok(r) => { r.zero(); Some(r) }
            Err(_) => { reply_with_status(tx, req, E_IO); return; }
        }
    } else {
        None
    };
    let (data_phys, data_len) = region.as_ref().map_or((0u64, 0u16), |r| (r.phys(), w_len));
    match do_transfer(ctx, slot, bm_rt, b_req, w_value, w_index, data_len, data_phys) {
        Ok(()) => send_reply(tx, req, data_len, region.as_ref()),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
fn do_transfer(ctx: &mut Context, slot: u8, bm_rt: u8, b_req: u8,
    w_value: u16, w_index: u16, data_len: u16, data_phys: u64,
) -> crate::error::XhciResult<()> {
    let (max_slots, db, intr) = (ctx.driver.layout.max_slots,
        ctx.driver.layout.doorbell_base, ctx.driver.layout.primary_intr_base);
    let res = ctx.driver.slots.resources_mut(slot, max_slots)
        .ok_or(crate::error::XhciError::ControllerUnsupported)?;
    issue_control_transfer(db, intr, &mut ctx.driver.event_ring, &mut res.ep0,
        slot, bm_rt, b_req, w_value, w_index, data_phys, data_len)
}
fn send_reply(tx: &mut [u8], req: &Request, actual: u16, region: Option<&DmaRegion>) {
    let plen = (STATUS_LEN + CONTROL_TRANSFER_REPLY_PREFIX + actual as usize) as u32;
    encode_response_header(tx, req, plen);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + 2].copy_from_slice(&actual.to_le_bytes());
    if let Some(r) = region {
        for i in 0..actual as usize {
            unsafe {
                tx[o + CONTROL_TRANSFER_REPLY_PREFIX + i] =
                    core::ptr::read_volatile(r.as_mut_ptr::<u8>().add(i));
            }
        }
    }
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + plen as usize);
}
