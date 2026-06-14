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

use crate::protocol::{E_OK, E_TIMEOUT, OP_CONNECT};
use crate::server::handlers::connect::{body, open, wait};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::server::tcp_tx;
use crate::tcp::FLAG_SYN;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (dst, dst_port) = match body::parse(body) {
        Ok(v) => v,
        Err(e) => return status(sender_pid, req, e, tx),
    };
    let Some((handle, tcb)) = open::connection(sender_pid, dst, dst_port) else {
        return status(sender_pid, req, E_TIMEOUT, tx);
    };
    if tcp_tx::send(tcb, FLAG_SYN, &[]).is_err() || !wait::established(sender_pid, handle) {
        crate::state::TABLE.lock().remove_by_handle(handle);
        return status(sender_pid, req, E_TIMEOUT, tx);
    }
    tx[20..24].copy_from_slice(&handle.to_le_bytes());
    let _ = respond(sender_pid, OP_CONNECT, E_OK, req.request_id, 4, tx);
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_CONNECT, errno, req.request_id, 0, tx);
}
