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

use alloc::vec;

use nonos_libc::{mk_ipc_recv, mk_irq_ack, mk_irq_poll, IrqPollOut};

use crate::protocol::{
    decode_request, Request, E_INVAL, HDR_LEN, MAX_RW_PAYLOAD_BYTES, OP_CAPACITY,
    OP_CONTROLLER_INFO, OP_FLUSH, OP_HEALTHCHECK, OP_PORT_LIST, OP_READ_BLOCKS, OP_WRITE_BLOCKS,
    RESP_HDR_LEN, SERVICE_NAME, STATUS_LEN,
};
use crate::server::{error, handlers};
use crate::setup::Driver;

pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let tx_len = RESP_HDR_LEN + STATUS_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    let mut last_irq_seq = 0u64;
    let _service_name = SERVICE_NAME;

    loop {
        poll_irq(driver, &mut last_irq_seq);
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), rx_len, 0);
        if n <= 0 {
            continue;
        }
        let req = match decode_request(&rx[..n as usize]) {
            Some(r) => r,
            None => {
                error::reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        let body_end = n as usize;
        dispatch(driver, &req, &rx[HDR_LEN..body_end], &mut tx);
    }
}

fn dispatch(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK | OP_CONTROLLER_INFO | OP_PORT_LIST if req.payload_len != 0 => {
            error::reply_with_status(tx, req, E_INVAL)
        }
        OP_HEALTHCHECK => handlers::health::handle(req, tx),
        OP_CONTROLLER_INFO => handlers::controller_info::handle(driver, req, tx),
        OP_PORT_LIST => handlers::port_list::handle(driver, req, tx),
        OP_CAPACITY => handlers::capacity::handle(driver, req, tx),
        OP_READ_BLOCKS => handlers::read::handle(driver, req, body, tx),
        OP_WRITE_BLOCKS => handlers::write::handle(driver, req, body, tx),
        OP_FLUSH => handlers::flush::handle(driver, req, tx),
        _ => error::reply_with_status(tx, req, E_INVAL),
    }
}

fn poll_irq(driver: &Driver, last: &mut u64) {
    let mut irq = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(driver.handles.irq_grant_id(), &mut irq as *mut _) >= 0 && irq.seq != *last {
        *last = irq.seq;
        let _ = mk_irq_ack(driver.handles.irq_grant_id());
    }
}
