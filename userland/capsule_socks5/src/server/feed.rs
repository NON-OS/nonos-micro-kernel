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
use crate::wire::method_reply;
use alloc::vec::Vec;

/// Drive the SOCKS handshake with bytes from the client, returning whatever
/// should go back.
///
/// A rejected or malformed client is answered and closed rather than left
/// half open: the state machine reports that through `is_closed`, and holding
/// the slot would deny it to the next caller.
pub fn feed(pid: u32, data: &[u8]) -> Vec<u8> {
    let mut guard = SERVER.lock();
    let Some(server) = guard.as_mut() else {
        return Vec::new();
    };
    let Some(conn) = server.clients.get(pid) else {
        // Table full. Refusing is the honest answer: the client disconnects
        // rather than waiting on a handshake that will never advance.
        return method_reply(false).to_vec();
    };
    match conn.on_client(data) {
        Event::NeedMore => Vec::new(),
        Event::ToClient { buf, len } => buf[..len].to_vec(),
        Event::Close => {
            server.clients.drop_client(pid);
            Vec::new()
        }
        Event::Open(dest) => {
            // A full table has no id to give. Zero is reserved for "no
            // connection", so using it would open a tunnel that every later
            // frame fails to match, rather than telling the client no.
            let opened = match server.manager.open(0) {
                Some(id) => open_tunnel(id, &dest) == OpenOutcome::Opened,
                None => false,
            };
            let Some(conn) = server.clients.get(pid) else {
                return Vec::new();
            };
            let (reply, len) = conn.opened(opened);
            reply[..len].to_vec()
        }
    }
}
