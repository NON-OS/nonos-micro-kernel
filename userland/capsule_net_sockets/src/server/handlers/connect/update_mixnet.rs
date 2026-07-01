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

use crate::clients::nym;
use crate::protocol::{E_NO_TRANSPORT, E_OK};
use crate::server::parse_req::Request;
use crate::sockets::{Kind, SocketKey};
use crate::state;

use super::{connect_nym, install_transport, status};

pub fn update_mixnet(pid: u32, req: &Request, key: SocketKey, ip: [u8; 4], port: u16, tx: &mut [u8]) {
    let transport = match connect_nym::connect_nym() {
        Ok(h) => h,
        Err(_) => return status::status(pid, req, E_NO_TRANSPORT, tx),
    };
    let errno = install_transport::install_transport(key, Kind::Mixnet, ip, port, transport);
    if errno != E_OK {
        let _ = nym::close(state::nym(), transport);
    }
    status::status(pid, req, errno, tx);
}
