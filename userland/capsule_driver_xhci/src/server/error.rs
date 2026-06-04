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
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use nonos_libc::mk_ipc_send;
pub fn reply_with_status(tx: &mut [u8], req: &Request, status: i32) {
    encode_response_header(tx, req, STATUS_LEN as u32);
    write_status(&mut tx[RESP_HDR_LEN..], status);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN);
}
pub fn reply_decode_failed(tx: &mut [u8], status: i32) {
    let req = Request { op: 0, flags: 0, request_id: 0, payload_len: 0 };
    reply_with_status(tx, &req, status);
}
