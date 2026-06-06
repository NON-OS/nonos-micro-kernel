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
use crate::setup::Driver;
use nonos_libc::mk_ipc_send;

pub(super) fn reply_read(driver: &mut Driver, req: &Request, tx: &mut [u8], bytes_n: usize) {
    let payload_len = STATUS_LEN as u32 + bytes_n as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let buf = unsafe { driver.queue.data(bytes_n as u32) };
    tx[RESP_HDR_LEN + STATUS_LEN..RESP_HDR_LEN + STATUS_LEN + bytes_n].copy_from_slice(buf);
    let len = RESP_HDR_LEN + STATUS_LEN + bytes_n;
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), len);
}
