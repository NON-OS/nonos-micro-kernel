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
    encode_response_header, write_status, Request, ALLOC_TRANSFER_RING_REPLY_LEN,
    KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};
use nonos_libc::mk_ipc_send;

pub(super) fn send_reply(tx: &mut [u8], req: &Request, dci: u8) {
    let plen = (STATUS_LEN + ALLOC_TRANSFER_RING_REPLY_LEN) as u32;
    encode_response_header(tx, req, plen);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o] = dci;
    tx[o + 1..o + ALLOC_TRANSFER_RING_REPLY_LEN].fill(0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + plen as usize);
}
