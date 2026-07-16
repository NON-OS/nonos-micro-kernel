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

//! The SOCKS5 method-selection handshake.

use super::{METHOD_NONE, METHOD_NO_ACCEPT, VER};

/// Parse the client greeting `[VER][NMETHODS][METHODS..]`. Returns whether the
/// no-authentication method is offered, or `None` if the buffer is too short or
/// is not a SOCKS5 greeting. This proxy only accepts no-auth: it runs on the
/// loopback for local clients, so the trust boundary is the capability system,
/// not a SOCKS password.
pub fn offers_no_auth(greeting: &[u8]) -> Option<bool> {
    if greeting.len() < 2 || greeting[0] != VER {
        return None;
    }
    let n = greeting[1] as usize;
    if greeting.len() < 2 + n {
        return None;
    }
    Some(greeting[2..2 + n].contains(&METHOD_NONE))
}

/// The server method-selection reply `[VER][METHOD]`: no-auth when the client
/// offered it, otherwise no-acceptable-methods (the client then disconnects).
pub fn method_reply(accepted: bool) -> [u8; 2] {
    [VER, if accepted { METHOD_NONE } else { METHOD_NO_ACCEPT }]
}
