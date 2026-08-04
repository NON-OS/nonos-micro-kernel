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

use super::api::base58::decode32;
use super::api::field::string_field;
use super::plain::fetch_plain;

/// Port a node answers questions about itself on.
const NODE_API_PORT: u16 = 8080;

/// Where a node publishes the requester that will open connections for us.
const REQUESTER_PATH: &str = "/api/v1/network-requester";

/// The exit that opens connections on our behalf.
pub struct ExitAddress {
    pub identity: [u8; 32],
    pub encryption: [u8; 32],
    pub gateway: [u8; 32],
}

/// Ask an exit node for the requester it runs.
///
/// Each node publishes this itself rather than the directory carrying it for
/// everyone, which is why this is asked node by node: the list that does
/// carry them all is megabytes, and a client that has already chosen an exit
/// does not need the rest.
///
/// The address is `identity.encryption@gateway`, and all three are needed.
/// The identity says who the request is for, the encryption key is what it is
/// sealed to, and the gateway is where a packet leaves the mixnet to reach
/// it.
pub fn fetch_exit(tcp_port: u32, ip: [u8; 4], gateway: [u8; 32]) -> Option<ExitAddress> {
    let body = fetch_plain(tcp_port, ip, NODE_API_PORT, REQUESTER_PATH).ok()?;
    let identity = decode32(string_field(&body, "encoded_identity_key")?.as_slice())?;
    let encryption = decode32(string_field(&body, "encoded_x25519_key")?.as_slice())?;
    Some(ExitAddress { identity, encryption, gateway })
}
