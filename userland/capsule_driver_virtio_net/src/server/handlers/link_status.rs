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

use crate::constants::VIRTIO_NET_S_LINK_UP;
use crate::protocol::{
    encode_response_header, write_status, Request, LINK_STATUS_PAYLOAD_LEN, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender_pid: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let up = if driver.status_supported {
        let s = unsafe { driver.regs.r16(driver.net_status_offset) };
        (s & VIRTIO_NET_S_LINK_UP) != 0
    } else {
        true
    };
    let payload_len = STATUS_LEN as u32 + LINK_STATUS_PAYLOAD_LEN as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN] = if up { 1 } else { 0 };
    reply(sender_pid, tx, RESP_HDR_LEN + STATUS_LEN + LINK_STATUS_PAYLOAD_LEN)
}
