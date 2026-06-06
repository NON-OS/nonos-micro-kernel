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

use nonos_libc::mk_ipc_recv;

use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{
    decode_request, E_INVAL, E_MSGSIZE, HDR_LEN, OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS,
    OP_RX_PACKET, OP_STATS, OP_TX_PACKET, RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN, STATS_PAYLOAD_LEN,
    STATUS_LEN,
};
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers;
use crate::setup::Driver;

const SERVICE_INBOX: u64 = 0;

pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + MAX_ETHERNET_FRAME;
    let tx_len = RESP_HDR_LEN
        + STATUS_LEN
        + core::cmp::max(RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME, STATS_PAYLOAD_LEN);
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];

    loop {
        let n = mk_ipc_recv(SERVICE_INBOX, rx.as_mut_ptr(), rx_len, 0);
        if n <= 0 {
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
        let body_end = HDR_LEN.saturating_add(req.payload_len as usize);
        if body_end != len {
            reply_with_status(&mut tx, &req, E_MSGSIZE);
            continue;
        }
        let body = &rx[HDR_LEN..body_end];
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_LINK_STATUS => handlers::link_status::handle(driver, &req, &mut tx),
            OP_MAC_ADDRESS => handlers::mac_address::handle(driver, &req, &mut tx),
            OP_TX_PACKET => handlers::tx_packet::handle(driver, &req, body, &mut tx),
            OP_RX_PACKET => handlers::rx_packet::handle(driver, &req, &mut tx),
            OP_STATS => handlers::stats::handle(driver, &req, &mut tx),
            _ => reply_with_status(&mut tx, &req, E_INVAL),
        }
    }
}
