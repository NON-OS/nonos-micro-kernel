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

use nonos_libc::mk_ipc_recv_from;

use crate::constants::{MAX_ETHERNET_FRAME, VIRTIO_NET_HDR_LEN};
use crate::protocol::{
    decode_request, E_INVAL, HDR_LEN, OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET,
    OP_TX_PACKET, RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN, STATUS_LEN,
};
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers;
use crate::setup::Driver;

pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + MAX_ETHERNET_FRAME;
    let tx_len =
        RESP_HDR_LEN + STATUS_LEN + RX_PAYLOAD_PREFIX_LEN + VIRTIO_NET_HDR_LEN + MAX_ETHERNET_FRAME;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];

    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let len = n as usize;
        let req = match decode_request(&rx[..len]) {
            Some(r) => r,
            None => {
                reply_decode_failed(sender_pid, &mut tx, E_INVAL);
                continue;
            }
        };
        let body = &rx[HDR_LEN..len];
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(sender_pid, &req, &mut tx),
            OP_LINK_STATUS => handlers::link_status::handle(sender_pid, driver, &req, &mut tx),
            OP_MAC_ADDRESS => handlers::mac_address::handle(sender_pid, driver, &req, &mut tx),
            OP_TX_PACKET => handlers::tx_packet::handle(sender_pid, driver, &req, body, &mut tx),
            OP_RX_PACKET => handlers::rx_packet::handle(sender_pid, driver, &req, &mut tx),
            _ => reply_with_status(sender_pid, &mut tx, &req, E_INVAL),
        };
    }
}
