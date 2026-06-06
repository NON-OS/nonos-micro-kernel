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
    decode_request, CONTROLLER_STATUS_PAYLOAD_LEN, EVENT_WIRE_LEN, E_INVAL, HDR_LEN,
    MAX_POLL_EVENTS, MOUSE_EVENT_WIRE_LEN, MOUSE_POLL_PAYLOAD_PREFIX_LEN, OP_CONTROLLER_STATUS,
    OP_GET_STATE, OP_HEALTHCHECK, OP_POLL_EVENTS, OP_POLL_MOUSE, POLL_PAYLOAD_PREFIX_LEN,
    RESP_HDR_LEN, STATE_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers;
use crate::server::pump::pump;
use crate::setup::Driver;
use alloc::vec;
use nonos_libc::mk_ipc_recv;

const POLL_IDLE_MS: u64 = 4;

pub fn run(driver: Driver) -> ! {
    let rx_len = HDR_LEN;
    let poll_tx_len = RESP_HDR_LEN + POLL_PAYLOAD_PREFIX_LEN + MAX_POLL_EVENTS * EVENT_WIRE_LEN;
    let mouse_tx_len =
        RESP_HDR_LEN + MOUSE_POLL_PAYLOAD_PREFIX_LEN + MAX_POLL_EVENTS * MOUSE_EVENT_WIRE_LEN;
    let state_tx_len = RESP_HDR_LEN + STATUS_LEN + STATE_PAYLOAD_LEN;
    let ctl_tx_len = RESP_HDR_LEN + STATUS_LEN + CONTROLLER_STATUS_PAYLOAD_LEN;
    let tx_len = core::cmp::max(
        core::cmp::max(poll_tx_len, mouse_tx_len),
        core::cmp::max(state_tx_len, ctl_tx_len),
    );
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    let mut ctx = Context::new(driver);
    let mut prev_buttons = 0u8;
    loop {
        pump(&mut ctx, &mut prev_buttons);
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), rx_len, POLL_IDLE_MS);
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
        if req.payload_len != 0 {
            reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_POLL_EVENTS => handlers::poll::handle(&mut ctx, &req, &mut tx),
            OP_POLL_MOUSE => handlers::mouse::handle(&mut ctx, &req, &mut tx),
            OP_GET_STATE => handlers::state::handle(&mut ctx, &req, &mut tx),
            OP_CONTROLLER_STATUS => handlers::controller_status::handle(&mut ctx, &req, &mut tx),
            _ => reply_with_status(&mut tx, &req, E_INVAL),
        }
    }
}
