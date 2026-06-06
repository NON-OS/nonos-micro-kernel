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
    encode_response_header, write_status, Request, E_AGAIN, E_IO, INTERRUPT_IN_REPLY_PREFIX,
    RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::SlotResources;
pub fn reply_pending(tx: &mut [u8], req: &Request) {
    let plen = (STATUS_LEN + INTERRUPT_IN_REPLY_PREFIX) as u32;
    encode_response_header(tx, req, plen);
    write_status(&mut tx[RESP_HDR_LEN..], E_AGAIN);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + INTERRUPT_IN_REPLY_PREFIX].fill(0);
    crate::server::reply::send(tx.as_ptr(), RESP_HDR_LEN + plen as usize);
}
pub fn reply_report(ctx: &mut Context, req: &Request, slot: u8, tx: &mut [u8], n: u16) {
    let max_slots = ctx.driver.layout.max_slots;
    let res = match ctx.driver.slots.resources_mut(slot, max_slots) {
        Some(r) => r,
        None => return reply_with_status(tx, req, E_IO),
    };
    emit_report(tx, req, res, n);
}
fn emit_report(tx: &mut [u8], req: &Request, res: &SlotResources, n: u16) {
    let buf = match res.int_buf.as_ref() {
        Some(b) => b,
        None => return reply_with_status(tx, req, E_IO),
    };
    let count = n as usize;
    let plen = (STATUS_LEN + INTERRUPT_IN_REPLY_PREFIX + count) as u32;
    encode_response_header(tx, req, plen);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + 2].copy_from_slice(&n.to_le_bytes());
    for i in 0..count {
        unsafe {
            tx[o + INTERRUPT_IN_REPLY_PREFIX + i] =
                core::ptr::read_volatile(buf.as_mut_ptr::<u8>().add(i));
        }
    }
    crate::server::reply::send(tx.as_ptr(), RESP_HDR_LEN + plen as usize);
}
