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
use crate::protocol::{
    encode_response_header, write_status, Request, ADDRESS_DEVICE_REPLY_LEN,
    KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
pub fn reply_ok(tx: &mut [u8], req: &Request, slot: u8, port: u8, speed: u8, max_packet: u16) {
    let payload_len = (STATUS_LEN + ADDRESS_DEVICE_REPLY_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o] = slot;
    tx[o + 1] = port;
    tx[o + 2] = speed;
    tx[o + 3] = 0;
    tx[o + 4..o + 6].copy_from_slice(&max_packet.to_le_bytes());
    tx[o + 6..o + ADDRESS_DEVICE_REPLY_LEN].fill(0);
    let len = RESP_HDR_LEN + payload_len as usize;
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), len);
}