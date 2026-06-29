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

use core::sync::atomic::Ordering;

use crate::protocol::{E_NO_CONFIG, E_RX_EMPTY, OP_POLL_PACKET};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::IFACE;

use super::{deliver, route, select};

const POLL_BUDGET: usize = 8;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let wanted = match select::wanted_protocol(body) {
        Ok(v) => v,
        Err(errno) => return status(sender_pid, req, errno, tx),
    };
    if let Some(p) = select::queued(wanted) {
        return deliver::send(sender_pid, req, p, tx);
    }
    let l2 = IFACE.l2_service_port.load(Ordering::Acquire);
    if l2 == 0 {
        return status(sender_pid, req, E_NO_CONFIG, tx);
    }
    for _ in 0..POLL_BUDGET {
        match route::poll_and_route(l2) {
            route::PollResult::KeepPolling => continue,
            route::PollResult::Empty => break,
            route::PollResult::Fault(errno) => return status(sender_pid, req, errno, tx),
        }
    }
    match select::queued(wanted) {
        Some(p) => deliver::send(sender_pid, req, p, tx),
        None => status(sender_pid, req, E_RX_EMPTY, tx),
    }
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_POLL_PACKET, errno, req.request_id, 0, tx);
}
