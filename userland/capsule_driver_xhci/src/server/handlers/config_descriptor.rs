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
use crate::controller::{get_config_descriptor, CONFIG_DESCRIPTOR_MAX};
use crate::protocol::{
    encode_response_header, write_status, Request, CONFIG_DESCRIPTOR_REPLY_PREFIX,
    CONFIG_DESCRIPTOR_REQUEST_LEN, E_INVAL, E_IO, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONFIG_DESCRIPTOR_REQUEST_LEN || body[0] == 0 || body[1] != 0 {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let requested = u16::from_le_bytes([body[2], body[3]]);
    let len = core::cmp::min(requested, CONFIG_DESCRIPTOR_MAX);
    if len == 0 {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let out = match ctx.driver.dma_pool.alloc(len as u64) {
        Ok(r) => r,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    out.zero();
    match transfer(ctx, body[0], &out, len) {
        Ok(actual) => reply_config(tx, req, &out, actual),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
fn transfer(
    ctx: &mut Context,
    slot: u8,
    out: &crate::dma::DmaRegion,
    len: u16,
) -> crate::error::XhciResult<usize> {
    let max_slots = ctx.driver.layout.max_slots;
    let resources = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .ok_or(crate::error::XhciError::ControllerUnsupported)?;
    get_config_descriptor(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.event_ring,
        slot,
        &mut resources.ep0,
        out,
        len,
    )
}
fn reply_config(tx: &mut [u8], req: &Request, out: &crate::dma::DmaRegion, actual: usize) {
    let payload_len = (STATUS_LEN + CONFIG_DESCRIPTOR_REPLY_PREFIX + actual) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + 2].copy_from_slice(&(actual as u16).to_le_bytes());
    tx[o + 2..o + CONFIG_DESCRIPTOR_REPLY_PREFIX].fill(0);
    for i in 0..actual {
        unsafe {
            tx[o + CONFIG_DESCRIPTOR_REPLY_PREFIX + i] =
                core::ptr::read_volatile(out.as_mut_ptr::<u8>().add(i));
        }
    }
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}