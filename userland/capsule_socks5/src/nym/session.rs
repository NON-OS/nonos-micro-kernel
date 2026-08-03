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

/// Placeholder for the open-session request until the IPC client lands.
fn request_open() -> Option<u32> {
    None
}
