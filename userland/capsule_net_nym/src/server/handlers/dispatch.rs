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

use super::{
    close, cover, gateway, get_exit, health, open, recv, send, send_reply, set_authority,
    set_credential, set_destination, set_identity, set_timing, set_topology, surb, sync_directory,
    timing_status, topology_status,
};
use crate::protocol::*;
use crate::server::parse_req::Request;

pub fn dispatch(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
    match req.op {
        OP_HEALTHCHECK => health::handle(pid, req, tx),
        OP_SET_GATEWAY => gateway::handle(pid, req, body, tx),
        OP_OPEN_SESSION => open::handle(pid, req, tx),
        OP_SEND => send::handle(pid, req, body, tx),
        OP_RECV => recv::handle(pid, req, body, tx),
        OP_COVER_TICK => cover::handle(pid, req, body, tx),
        OP_CLOSE => close::handle(pid, req, body, tx),
        OP_SET_TOPOLOGY => set_topology::handle(pid, req, body, tx),
        OP_SET_CREDENTIAL => set_credential::handle(pid, req, body, tx),
        OP_CREATE_SURB => surb::handle(pid, req, body, tx),
        OP_SEND_REPLY => send_reply::handle(pid, req, body, tx),
        OP_SET_DESTINATION => set_destination::handle(pid, req, body, tx),
        OP_GET_EXIT => get_exit::handle(pid, req, body, tx),
        OP_SET_IDENTITY => set_identity::handle(pid, req, body, tx),
        OP_SET_TIMING => set_timing::handle(pid, req, body, tx),
        OP_SET_AUTHORITY => set_authority::handle(pid, req, body, tx),
        OP_SYNC_DIRECTORY => sync_directory::handle(pid, req, body, tx),
        OP_TOPOLOGY_STATUS => topology_status::handle(pid, req, tx),
        OP_TIMING_STATUS => timing_status::handle(pid, req, tx),
        _ => return false,
    }
    true
}
