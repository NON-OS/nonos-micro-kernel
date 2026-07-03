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
use crate::sockets::{Kind, RemoteAddr4, SocketKey, SOCKETS};

pub fn install_transport(
    key: SocketKey,
    kind: Kind,
    ip: [u8; 4],
    port: u16,
    transport: u32,
) -> u16 {
    match SOCKETS.with(key, |s| {
        if s.kind != kind {
            return E_NO_HANDLE;
        }
        s.remote = Some(RemoteAddr4 { ip, port });
        s.transport_handle = transport;
        E_OK
    }) {
        Some(errno) => errno,
        None => E_NO_HANDLE,
    }
}
