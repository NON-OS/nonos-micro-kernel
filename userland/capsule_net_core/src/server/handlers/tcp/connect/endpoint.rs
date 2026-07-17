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

use smoltcp::wire::{IpAddress, Ipv4Address};

use crate::protocol::tcp::{E_BAD_ADDR, E_BAD_LEN};
use crate::server::handlers::tcp::connect::types::Endpoint;

pub fn parse(body: &[u8]) -> Result<Endpoint, u16> {
    if body.len() < 6 {
        return Err(E_BAD_LEN);
    }
    let ip = [body[0], body[1], body[2], body[3]];
    let port = u16::from_le_bytes([body[4], body[5]]);
    if ip == [0, 0, 0, 0] || port == 0 {
        return Err(E_BAD_ADDR);
    }
    Ok(Endpoint { remote: IpAddress::Ipv4(Ipv4Address(ip)), port })
}
