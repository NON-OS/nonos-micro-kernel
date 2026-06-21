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

//! `OP_MAC_ADDRESS`. Returns the 6 bytes the driver cached at
//! bring-up. The kernel client treats an all-zero MAC as a hard
//! error so a misprogrammed RAL/RAH never silently passes a validation.

use nonos_libc::mk_ipc_send;

use crate::protocol::{
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, MAC_ADDRESS_PAYLOAD_LEN,
    RESP_HDR_LEN, STATUS_LEN,
};
use crate::setup::Driver;

pub fn handle(driver: &Driver, req: &Request, tx: &mut [u8]) {
    let payload_len = STATUS_LEN as u32 + MAC_ADDRESS_PAYLOAD_LEN as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN..RESP_HDR_LEN + STATUS_LEN + MAC_ADDRESS_PAYLOAD_LEN]
        .copy_from_slice(&driver.mac);
    let _ = mk_ipc_send(
        KERNEL_REPLY_ENDPOINT,
        tx.as_ptr(),
        RESP_HDR_LEN + STATUS_LEN + MAC_ADDRESS_PAYLOAD_LEN,
    );
}
