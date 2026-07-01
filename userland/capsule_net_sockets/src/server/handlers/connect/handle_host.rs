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

use crate::protocol::{E_BAD_ADDR, E_BAD_LEN};
use crate::server::parse_req::Request;
use crate::sockets::SocketKey;

use super::{finish_host, parse_host, resolve_host, status_host};

pub fn handle_host(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, port, host) = match parse_host::parse(body) {
        Some(v) => v,
        None => return status_host::status(pid, req, E_BAD_LEN, tx),
    };
    let Some(ip) = resolve_host::resolve_host(host) else {
        return status_host::status(pid, req, E_BAD_ADDR, tx);
    };
    let errno = finish_host::finish(SocketKey { pid, handle }, ip, port);
    status_host::status(pid, req, errno, tx);
}
