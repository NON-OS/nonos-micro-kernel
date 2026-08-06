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

use super::exit::Exit;
use crate::ipc::{call, OP_GET_EXIT};

/// Ask the mixnet capsule for an exit the directory published.
///
/// Preferred over anything compiled in, because a compiled list ages: an
/// operator who stops running a requester leaves every client that shipped
/// with it unable to reach anything, and no update can reach a client that
/// cannot reach the network.
///
/// The answer is the exit's identity, its encryption key, and the gateway it
/// sits behind. All three are needed and none is derivable from the others.
pub fn discover_exit(index: u32) -> Option<Exit> {
    let reply = call(OP_GET_EXIT, &index.to_le_bytes()).ok()?;
    if reply.len() < 96 {
        return None;
    }
    let mut exit = Exit { identity: [0u8; 32], encryption: [0u8; 32], gateway: [0u8; 32] };
    exit.identity.copy_from_slice(&reply[..32]);
    exit.encryption.copy_from_slice(&reply[32..64]);
    exit.gateway.copy_from_slice(&reply[64..96]);
    // An all zero key is what an unset field looks like, and sealing to it
    // would put the request on the wire readable by anyone who noticed.
    if exit.encryption == [0u8; 32] || exit.identity == [0u8; 32] {
        return None;
    }
    Some(exit)
}
