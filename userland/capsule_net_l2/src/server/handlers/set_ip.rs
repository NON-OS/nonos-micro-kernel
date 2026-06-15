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

use crate::protocol::{Request, E_BAD_LEN, E_OK, OP_SET_IP};
use crate::server::respond::respond;
use crate::state::STATE;

// Body layout (request): 4 bytes interface IPv4. The DHCP client
// pushes the leased address here once a lease binds, so L2 can put
// a real sender IP into outbound ARP and answer ARP-for-host.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != 4 {
        let _ = respond(sender_pid, OP_SET_IP, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let mut ipv4 = [0u8; 4];
    ipv4.copy_from_slice(body);
    *STATE.ipv4.lock() = ipv4;
    let _ = respond(sender_pid, OP_SET_IP, E_OK, req.request_id, 0, tx);
}
