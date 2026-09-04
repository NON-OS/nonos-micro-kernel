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

use super::bind::bind_destination;
use super::exit::exit;
use crate::ipc::{call, OP_CLOSE, OP_OPEN_SESSION};
use crate::setup::nym_port;
use core::sync::atomic::{AtomicU32, Ordering};

/// The `net.nym` session this capsule sends through.
static SESSION: AtomicU32 = AtomicU32::new(0);

/// Ask `net.nym` to open a session and remember its id.
///
/// One session serves every SOCKS connection. The mixnet already unlinks
/// traffic at the packet level, and a session per connection would multiply
/// gateway registrations without adding anonymity.
pub fn open_session() -> Option<u32> {
    let existing = SESSION.load(Ordering::Acquire);
    if existing != 0 {
        return Some(existing);
    }
    if nym_port() == 0 {
        return None;
    }
    let id = request_open()?;
    SESSION.store(id, Ordering::Release);
    Some(id)
}

pub fn session() -> Option<u32> {
    match SESSION.load(Ordering::Acquire) {
        0 => None,
        id => Some(id),
    }
}

/// Close the mixnet session and forget it, so the next open builds a fresh
/// one against the current exit.
///
/// Used when the exit is rotated. Closing on net.nym is not optional: it
/// holds one destination at a time and refuses a second as busy, so an old
/// session left open would make every rebind to the replacement exit fail
/// and the rotation would fix nothing.
pub fn reset_session() {
    if let Some(id) = session() {
        let _ = call(OP_CLOSE, &id.to_le_bytes());
    }
    SESSION.store(0, Ordering::Release);
}

/// Open a session and bind it to the configured exit.
///
/// Binding is part of opening rather than a later step: a session with no
/// destination cannot be sealed as a Sphinx packet, and leaving one half
/// configured invites a send that fails at the last moment instead of here.
fn request_open() -> Option<u32> {
    let Some(exit) = exit() else {
        crate::server::trace_open(b"session no exit", 0);
        return None;
    };
    let reply = match call(OP_OPEN_SESSION, &[]) {
        Ok(reply) => reply,
        Err(crate::ipc::CallError::Remote(code)) => {
            crate::server::trace_open(b"session refused", code);
            return None;
        }
        Err(_) => {
            crate::server::trace_open(b"session no answer", 0);
            return None;
        }
    };
    if reply.len() < 4 {
        crate::server::trace_open(b"session short reply", reply.len() as u16);
        return None;
    }
    let id = u32::from_le_bytes([reply[0], reply[1], reply[2], reply[3]]);
    if bind_destination(id, &exit).is_err() {
        crate::server::trace_open(b"session bind failed", 0);
        return None;
    }
    Some(id)
}
