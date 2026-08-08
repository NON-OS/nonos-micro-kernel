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

use alloc::vec::Vec;

use super::session::session;
use crate::ipc::{call, CallError, OP_RECV};

/// What net.nym answers when nothing has come back yet. Not a failure: a
/// mixnet reply is seconds behind the request that asked for it, and treating
/// an empty queue as an error would close the tunnel on the way there.
const E_RX_EMPTY: u16 = 10;

pub enum Delivery {
    /// One message the mixnet delivered.
    Message(Vec<u8>),
    /// Nothing has arrived yet. Ask again.
    Empty,
    /// The transport is gone, so waiting longer will not help.
    Gone,
}

/// Take one message off the session, if the mixnet has delivered one.
pub fn recv_once() -> Delivery {
    let Some(id) = session() else { return Delivery::Gone };
    match call(OP_RECV, &id.to_le_bytes()) {
        Ok(body) => Delivery::Message(body),
        Err(CallError::Remote(E_RX_EMPTY)) => Delivery::Empty,
        // A call that did not come back says nothing about the tunnel. The
        // capsule may simply have been busy on a gateway read, so this waits
        // rather than tearing a live connection down.
        Err(CallError::Transport) => Delivery::Empty,
        Err(_) => Delivery::Gone,
    }
}
