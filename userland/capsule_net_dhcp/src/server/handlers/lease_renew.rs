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
use crate::dora::{install, request, InstallError, RequestError};
use crate::protocol::{E_NAK, E_NO_LINK, E_OK, E_TIMEOUT, OP_LEASE_RENEW};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::STATE;

use super::xid_mac::current;

// Renew the existing lease by reissuing a DHCPREQUEST for the
// already-bound yiaddr against the original server_id. On NAK the
// state collapses to Init; the caller should follow up with a
// fresh LEASE_REQUEST.
pub fn handle(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let (l2, ip, mac) = match current() {
        Some(v) => v,
        None => {
            let _ = respond(sender_pid, OP_LEASE_RENEW, E_NO_LINK, req.request_id, 0, tx);
            return;
        }
    };
    let prior = *STATE.lease.lock();
    if prior.ipv4 == [0; 4] {
        let _ = respond(sender_pid, OP_LEASE_RENEW, E_NO_LINK, req.request_id, 0, tx);
        return;
    }
    *STATE.client_state.lock() = ClientState::Renewing;
    let xid = match STATE.next_xid() {
        Some(x) => x,
        None => {
            let _ = respond(sender_pid, OP_LEASE_RENEW, E_NO_LINK, req.request_id, 0, tx);
            return;
        }
    };
    let mut msg = Message::new_request(&mac, xid);
    msg.ciaddr = prior.ipv4;
    let synthetic_offer = Message { yiaddr: prior.ipv4, server_id: prior.server_id, ..msg };
    let ack = match request(l2, &msg, &synthetic_offer) {
        Ok(m) => m,
        Err(RequestError::Nak) => {
            *STATE.client_state.lock() = ClientState::Init;
            let _ = respond(sender_pid, OP_LEASE_RENEW, E_NAK, req.request_id, 0, tx);
            return;
        }
        Err(RequestError::Wait(_)) => {
            let _ = respond(sender_pid, OP_LEASE_RENEW, E_TIMEOUT, req.request_id, 0, tx);
            return;
        }
        Err(RequestError::Send(_)) => {
            let _ = respond(sender_pid, OP_LEASE_RENEW, E_NO_LINK, req.request_id, 0, tx);
            return;
        }
    };
    if let Err(InstallError::BadMask) | Err(InstallError::IpRefused) = install(ip, &ack) {
        let _ = respond(sender_pid, OP_LEASE_RENEW, E_NO_LINK, req.request_id, 0, tx);
        return;
    }
    let mut lease = STATE.lease.lock();
    lease.lease_seconds = ack.lease_seconds;
    drop(lease);
    *STATE.client_state.lock() = ClientState::Bound;
    let _ = respond(sender_pid, OP_LEASE_RENEW, E_OK, req.request_id, 0, tx);
}
