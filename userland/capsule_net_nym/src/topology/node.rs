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

use super::types::{Node, Role};

pub fn parse(chunk: &[u8]) -> Option<Node> {
    let role = match chunk[0] {
        1 => Role::EntryGateway,
        2 => Role::Mix,
        3 => Role::ExitGateway,
        _ => return None,
    };
    let mut ip = [0u8; 4];
    let mut identity = [0u8; 32];
    let mut packet_key = [0u8; 32];
    ip.copy_from_slice(&chunk[4..8]);
    identity.copy_from_slice(&chunk[10..42]);
    packet_key.copy_from_slice(&chunk[42..74]);
    let ws_port = u16::from_le_bytes([chunk[74], chunk[75]]);
    Some(Node {
        role,
        layer: chunk[1],
        delay_ms: u16::from_le_bytes([chunk[2], chunk[3]]),
        ip,
        port: u16::from_le_bytes([chunk[8], chunk[9]]),
        ws_port,
        identity,
        packet_key,
    })
}
