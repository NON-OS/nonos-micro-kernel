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

use crate::server::parse_req::IPC_BUF_MAX;

/// What a caller could hold before this field existed, and so the most that
/// may be handed to one that does not send it.
const ASSUMED_CAP: usize = 1024;

/// How many bytes a receive may drain for this caller.
///
/// A read takes the bytes out of the socket, so anything sent back that the
/// caller cannot hold is not delayed but lost: the kernel copies what fits
/// and the rest is gone from a stream that has no way to ask for it again.
/// The caller states its own capacity, and a caller too old to state one is
/// held to what it was built against.
pub fn recv_cap(body: &[u8]) -> usize {
    let stated = if body.len() >= 8 {
        u32::from_le_bytes([body[4], body[5], body[6], body[7]]) as usize
    } else {
        ASSUMED_CAP
    };
    stated.clamp(1, IPC_BUF_MAX)
}
