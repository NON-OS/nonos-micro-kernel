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
    encode_response_header, write_status, Request, CONTROLLER_STATUS_PAYLOAD_LEN,
    KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::regs::op::{usbcmd_read, usbsts_read};
use crate::regs::runtime::iman_read;
use crate::server::context::Context;
use nonos_libc::mk_ipc_send;
pub fn handle(ctx: &Context, req: &Request, tx: &mut [u8]) {
    let usbsts = usbsts_read(ctx.driver.layout.op_base);
    let usbcmd = usbcmd_read(ctx.driver.layout.op_base);
    let iman = iman_read(ctx.driver.layout.primary_intr_base);
    let cmd_cycle = ctx.driver.command_ring.cycle();
    let events_total = ctx.events_drained_total;
    let dcbaa_phys = ctx.driver.dcbaa.phys();
    let scratchpad_phys = ctx.driver.scratchpads.array_phys();
    let scratchpad_pages = ctx.driver.scratchpads.page_count();
    let allocated_slots = ctx.driver.slots.count() as u32;
    let payload_len = (STATUS_LEN + CONTROLLER_STATUS_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + STATUS_LEN;
    tx[o] = ctx.driver.layout.max_slots;
    tx[o + 1] = ctx.driver.layout.max_ports;
    tx[o + 2] = 0;
    tx[o + 3] = 0;
    o += 4;
    tx[o..o + 4].copy_from_slice(&ctx.driver.layout.max_scratchpad.to_le_bytes());
    o += 4;
    tx[o..o + 4].copy_from_slice(&scratchpad_pages.to_le_bytes());
    o += 4;
    tx[o..o + 4].copy_from_slice(&usbsts.to_le_bytes());
    o += 4;
    tx[o..o + 4].copy_from_slice(&usbcmd.to_le_bytes());
    o += 4;
    tx[o..o + 4].copy_from_slice(&iman.to_le_bytes());
    o += 4;
    tx[o] = cmd_cycle;
    tx[o + 1] = 0;
    tx[o + 2] = 0;
    tx[o + 3] = 0;
    o += 4;
    tx[o..o + 8].copy_from_slice(&events_total.to_le_bytes());
    o += 8;
    tx[o..o + 8].copy_from_slice(&dcbaa_phys.to_le_bytes());
    o += 8;
    tx[o..o + 8].copy_from_slice(&scratchpad_phys.to_le_bytes());
    o += 8;
    tx[o..o + 4].copy_from_slice(&allocated_slots.to_le_bytes());
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + (payload_len as usize));
}
