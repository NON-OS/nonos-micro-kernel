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

use super::open::{open_tunnel, OpenOutcome};
use super::state::SERVER;
use crate::conn::Event;
use alloc::vec::Vec;

/// Drive the SOCKS handshake with bytes from the client, returning whatever
/// should go back.
///
/// A rejected or malformed client is answered and closed rather than left
/// half open: the state machine reports that through `is_closed`, and holding
/// the slot would deny it to the next caller.
pub fn feed(data: &[u8]) -> Vec<u8> {
    let mut guard = SERVER.lock();
    let Some(server) = guard.as_mut() else {
        return Vec::new();
    };
    match server.conn.on_client(data) {
        Event::NeedMore => Vec::new(),
        Event::ToClient { buf, len } => buf[..len].to_vec(),
        Event::Close => {
            drop(guard);
            super::state::reset();
            Vec::new()
        }
        Event::Open(dest) => {
            let id = server.manager.open(0).unwrap_or(0);
            let outcome = open_tunnel(id, &dest);
            let (reply, len) = server.conn.opened(outcome == OpenOutcome::Opened);
            reply[..len].to_vec()
        }
    }
}
