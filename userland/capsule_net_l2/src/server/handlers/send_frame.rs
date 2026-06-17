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

use crate::nic_client::send_frame as nic_send;
use crate::protocol::{Request, E_NO_LINK, E_OK, E_PERM, E_TX_BUSY, OP_SEND_FRAME};
use crate::server::authz::authorized_any;
use crate::server::respond::respond_status_only;
use crate::state::STATE;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !authorized_any(sender_pid, &[b"net.ip", b"net.dhcp.client"]) {
        let _ = respond_status_only(sender_pid, OP_SEND_FRAME, E_PERM, req.request_id, tx);
        return;
    }
    let nic = STATE.nic_port();
    if nic == 0 {
        let _ = respond_status_only(sender_pid, OP_SEND_FRAME, E_NO_LINK, req.request_id, tx);
        return;
    }
    match nic_send(nic, body) {
        Ok(()) => {
            let _ = respond_status_only(sender_pid, OP_SEND_FRAME, E_OK, req.request_id, tx);
        }
        Err(_) => {
            let _ = respond_status_only(sender_pid, OP_SEND_FRAME, E_TX_BUSY, req.request_id, tx);
        }
    }
}
