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
    decode_request, E_INVAL, HDR_LEN, MAX_RW_PAYLOAD_BYTES, OP_CAPACITY, OP_FLUSH, OP_HEALTHCHECK,
    OP_READ_BLOCKS, OP_WRITE_BLOCKS, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::acl;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers;
use crate::setup::Driver;
use alloc::vec;
use nonos_libc::{mk_ipc_recv_from, mk_yield};
pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let tx_len = RESP_HDR_LEN + STATUS_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if n <= 0 {
            mk_yield();
            continue;
        }
        let len = n as usize;
        let req = match decode_request(&rx[..len]) {
            Some(r) => r,
            None => {
                reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        if !acl::permits(req.op, sender_pid) {
            let _ = reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        let body = &rx[HDR_LEN..len];
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_CAPACITY => handlers::capacity::handle(driver, &req, &mut tx),
            OP_READ_BLOCKS => handlers::read::handle(driver, &req, body, &mut tx),
            OP_WRITE_BLOCKS => handlers::write::handle(driver, &req, body, &mut tx),
            OP_FLUSH => handlers::flush::handle(driver, &req, &mut tx),
            _ => {
                let _ = reply_with_status(&mut tx, &req, E_INVAL);
            }
        }
    }
}
