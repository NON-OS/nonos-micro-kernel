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

use crate::clients::{nym, tcp};
use crate::protocol::{E_NO_HANDLE, E_NO_TRANSPORT, E_OK};
use crate::sockets::{Kind, RemoteAddr4, SocketKey, SOCKETS};
use crate::state;

use super::{connect_nym, install_transport, wait_established};

pub fn finish(key: SocketKey, ip: [u8; 4], port: u16) -> u16 {
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return E_NO_HANDLE;
    };
    match sock.kind {
        Kind::Datagram => SOCKETS
            .with(key, |s| s.remote = Some(RemoteAddr4 { ip, port }))
            .map_or(E_NO_HANDLE, |_| E_OK),
        Kind::Stream => match tcp::connect(state::tcp(), ip, port) {
            Ok(h) => {
                if !wait_established::wait_established(state::tcp(), h) {
                    let _ = tcp::close(state::tcp(), h);
                    return E_NO_TRANSPORT;
                }
                install_transport::install_transport(key, Kind::Stream, ip, port, h)
            }
            Err(_) => E_NO_TRANSPORT,
        },
        Kind::Mixnet => match connect_nym::connect_nym() {
            Ok(h) => {
                let e = install_transport::install_transport(key, Kind::Mixnet, ip, port, h);
                if e != E_OK {
                    let _ = nym::close(state::nym(), h);
                }
                e
            }
            Err(_) => E_NO_TRANSPORT,
        },
    }
}
