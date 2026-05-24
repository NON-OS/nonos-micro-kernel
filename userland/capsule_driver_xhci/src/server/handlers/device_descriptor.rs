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
use crate::controller::{get_device_descriptor, DEVICE_DESCRIPTOR_LEN};
use crate::protocol::{
    encode_response_header, write_status, Request, DEVICE_DESCRIPTOR_REPLY_LEN,
    DEVICE_DESCRIPTOR_REQUEST_LEN, E_INVAL, E_IO, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != DEVICE_DESCRIPTOR_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let slot = body[0];
    let out = match ctx.driver.dma_pool.alloc(DEVICE_DESCRIPTOR_LEN as u64) {
        Ok(r) => r,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    out.zero();
    if transfer(ctx, slot, &out).is_err() {
        reply_with_status(tx, req, E_IO);
        return;
    }
    reply_descriptor(tx, req, &out);
}
fn transfer(
    ctx: &mut Context,
    slot: u8,
    out: &crate::dma::DmaRegion,
) -> crate::error::XhciResult<usize> {
    let doorbell = ctx.driver.layout.doorbell_base;
    let intr = ctx.driver.layout.primary_intr_base;
    let max_slots = ctx.driver.layout.max_slots;
    let resources = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .ok_or(crate::error::XhciError::ControllerUnsupported)?;
    get_device_descriptor(doorbell, intr, &mut ctx.driver.event_ring, slot, &mut resources.ep0, out)
}
fn reply_descriptor(tx: &mut [u8], req: &Request, out: &crate::dma::DmaRegion) {
    let payload_len = (STATUS_LEN + DEVICE_DESCRIPTOR_REPLY_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let offset = RESP_HDR_LEN + STATUS_LEN;
    for i in 0..DEVICE_DESCRIPTOR_REPLY_LEN {
        unsafe {
            tx[offset + i] = core::ptr::read_volatile(out.as_mut_ptr::<u8>().add(i));
        }
    }
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}