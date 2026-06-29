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

use crate::protocol::{E_NO_HANDLE, E_OK, FLAG_NONBLOCK, OP_GETFLAGS};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{SocketKey, SOCKETS};

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(h) => h,
        Err(e) => {
            let _ = respond(pid, OP_GETFLAGS, e, req.request_id, 0, tx);
            return;
        }
    };
    let key = SocketKey { pid, handle };
    match SOCKETS.with(key, |s| if s.nonblock { FLAG_NONBLOCK } else { 0 }) {
        Some(flags) => {
            tx[20..24].copy_from_slice(&flags.to_le_bytes());
            let _ = respond(pid, OP_GETFLAGS, E_OK, req.request_id, 4, tx);
        }
        None => {
            let _ = respond(pid, OP_GETFLAGS, E_NO_HANDLE, req.request_id, 0, tx);
        }
    }
}
