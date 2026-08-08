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

use crate::protocol::{E_BAD_OP, IPC_PAYLOAD_MAX};

use super::handlers;
use super::parse_req::{parse, HDR_LEN};
use super::respond::respond;

const SERVICE_INBOX: u64 = 0;
/// How long to wait for a request before spending the gap on the mixnet.
/// Long enough that an idle capsule is not spinning, short enough that a
/// client arriving early is not left behind a gateway it never asked about.
const IDLE_MS: u64 = 400;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), IDLE_MS, &mut sender);
        if n <= 0 || sender == 0 {
            // Nothing to serve, so spend the gap on the work that has to
            // happen before anything can be served.
            //
            // The directory comes first. It is fetched over net.tcp and needs
            // no gateway, and holding a session across a fetch is what loses
            // one: a fetch takes seconds, the keepalive falls due while it
            // runs, and the ping afterwards finds a gateway that has closed
            // an idle connection. Fetching before there is a session to lose
            // avoids that, and the first gateway dialled is then one the
            // directory describes, which is the only kind a route home can
            // end at.
            super::directory_tick::directory_tick();
            super::connect_tick::connect_tick();
            super::keepalive::keepalive_tick();
            // Last, because the two above are what create a link to read.
            super::pump_tick::pump_tick();
            continue;
        }
        let Ok((req, body)) = parse(&rx[..n as usize]) else { continue };
        if !handlers::dispatch(sender, &req, body, &mut tx) {
            respond(sender, req.op, E_BAD_OP, req.request_id, 0, &mut tx);
        }
    }
}
