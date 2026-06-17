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

use crate::dhcp::{Message, State as ClientState};
use crate::dora::release;
use crate::ip_client::clear_lease;
use crate::protocol::{E_NO_LINK, E_OK, OP_LEASE_RELEASE};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{Lease, STATE};

use super::xid_mac::current;

// Send a DHCPRELEASE if a lease is bound, clear it in net.ip, and
// reset capsule state. A "release with no lease" succeeds idempotently.
pub fn handle(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let (l2, ip, mac) = match current() {
        Some(v) => v,
        None => {
            let _ = respond(sender_pid, OP_LEASE_RELEASE, E_NO_LINK, req.request_id, 0, tx);
            return;
        }
    };
    let prior = *STATE.lease.lock();
    if prior.ipv4 != [0; 4] {
        if let Some(xid) = STATE.next_xid() {
            let mut msg = Message::new_request(&mac, xid);
            msg.ciaddr = prior.ipv4;
            let _ = release(l2, &msg, prior.server_id);
        }
        let _ = clear_lease(ip);
    }
    *STATE.lease.lock() = Lease::empty();
    *STATE.client_state.lock() = ClientState::Init;
    let _ = respond(sender_pid, OP_LEASE_RELEASE, E_OK, req.request_id, 0, tx);
}
