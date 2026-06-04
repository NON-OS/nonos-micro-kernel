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
    encode_response_header, write_status, Request, E_AGAIN, RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN,
    STATUS_LEN,
};
use crate::rx::take_one;
use crate::server::error::{reply, reply_with_status};
use crate::setup::Driver;

pub fn handle(sender_pid: u32, driver: &mut Driver, req: &Request, tx: &mut [u8]) -> bool {
    let frame = unsafe { take_one(&mut driver.rx) };
    let frame = match frame {
        Some(f) => f,
        None => {
            return reply_with_status(sender_pid, tx, req, E_AGAIN);
        }
    };
    let len = frame.bytes.len();
    let body_len = RX_PAYLOAD_PREFIX_LEN + len;
    let payload_len = STATUS_LEN as u32 + body_len as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let prefix = (len as u32).to_le_bytes();
    tx[RESP_HDR_LEN + STATUS_LEN..RESP_HDR_LEN + STATUS_LEN + RX_PAYLOAD_PREFIX_LEN]
        .copy_from_slice(&prefix);
    tx[RESP_HDR_LEN + STATUS_LEN + RX_PAYLOAD_PREFIX_LEN..RESP_HDR_LEN + STATUS_LEN + body_len]
        .copy_from_slice(frame.bytes);
    reply(sender_pid, tx, RESP_HDR_LEN + STATUS_LEN + body_len)
}
