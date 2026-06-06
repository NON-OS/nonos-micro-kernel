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

use crate::constants::regs::REG_RDT;
use crate::protocol::{
    encode_response_header, write_status, Request, E_AGAIN, E_IO, KERNEL_REPLY_ENDPOINT,
    RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN, STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let (idx, len) = match driver.rx.consume() {
        Some(p) => p,
        None => {
            reply_with_status(tx, req, E_AGAIN);
            return;
        }
    };
    if len == 0 {
        unsafe {
            driver.regs.w32(REG_RDT, idx as u32);
        }
        reply_with_status(tx, req, E_IO);
        return;
    }
    let body_len = RX_PAYLOAD_PREFIX_LEN + len as usize;
    let payload_len = STATUS_LEN as u32 + body_len as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let prefix = (len as u32).to_le_bytes();
    let prefix_off = RESP_HDR_LEN + STATUS_LEN;
    let body_off = prefix_off + RX_PAYLOAD_PREFIX_LEN;
    tx[prefix_off..body_off].copy_from_slice(&prefix);
    let src = driver.rx.buffer_va(idx) as *const u8;
    unsafe {
        core::ptr::copy_nonoverlapping(src, tx[body_off..].as_mut_ptr(), len as usize);
    }
    unsafe {
        driver.regs.w32(REG_RDT, idx as u32);
    }
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), prefix_off + body_len);
}
