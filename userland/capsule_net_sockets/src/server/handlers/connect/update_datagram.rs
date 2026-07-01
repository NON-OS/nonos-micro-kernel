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

use crate::protocol::{E_NO_HANDLE, E_OK};
use crate::server::parse_req::Request;
use crate::sockets::{RemoteAddr4, SocketKey, SOCKETS};

use super::status;

pub fn update_datagram(pid: u32, req: &Request, key: SocketKey, ip: [u8; 4], port: u16, tx: &mut [u8]) {
    let errno = SOCKETS.with(key, |s| {
        s.remote = Some(RemoteAddr4 { ip, port });
        E_OK
    });
    status::status(pid, req, match errno {
        Some(value) => value,
        None => E_NO_HANDLE,
    }, tx);
}
