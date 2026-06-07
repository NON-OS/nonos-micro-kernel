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
    encode_response_header, write_status, Request, MAX_PORTS_REPORTED, PORT_ENTRY_BYTES,
    PORT_STATUS_HEADER_BYTES, RESP_HDR_LEN, STATUS_LEN,
};
use crate::regs::op::{portsc_clear_changes, portsc_read};
use crate::server::context::Context;
pub fn handle(ctx: &Context, req: &Request, tx: &mut [u8]) {
    let max_ports = ctx.driver.layout.max_ports as usize;
    let port_count = core::cmp::min(max_ports, MAX_PORTS_REPORTED);
    let payload_bytes = STATUS_LEN + PORT_STATUS_HEADER_BYTES + port_count * PORT_ENTRY_BYTES;
    encode_response_header(tx, req, payload_bytes as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + STATUS_LEN;
    tx[o] = port_count as u8;
    tx[o + 1] = 0;
    tx[o + 2] = 0;
    tx[o + 3] = 0;
    o += PORT_STATUS_HEADER_BYTES;
    for port_index in 0..port_count {
        let port_id = (port_index + 1) as u8;
        let portsc = portsc_read(ctx.driver.layout.op_base, port_id);
        portsc_clear_changes(ctx.driver.layout.op_base, port_id, portsc);
        tx[o] = port_id;
        tx[o + 1] = 0;
        tx[o + 2] = 0;
        tx[o + 3] = 0;
        tx[o + 4..o + 8].copy_from_slice(&portsc.to_le_bytes());
        o += PORT_ENTRY_BYTES;
    }
    crate::server::reply::send(tx.as_ptr(), RESP_HDR_LEN + payload_bytes);
}
