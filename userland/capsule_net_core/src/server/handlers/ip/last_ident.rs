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

use core::sync::atomic::{AtomicU32, Ordering};

/// The identifier the last send bound the socket to.
///
/// A poll carries only a protocol byte, so it has no identifier of its own to
/// bind with. Remembering what was last sent is what lets a poll reach the
/// same socket the request went out on. Zero means nothing has been sent yet.
static LAST: AtomicU32 = AtomicU32::new(0);

pub fn remember_ident(ident: u16) {
    LAST.store(ident as u32, Ordering::Relaxed);
}

pub fn last_ident() -> Option<u16> {
    match LAST.load(Ordering::Relaxed) {
        0 => None,
        v => Some(v as u16),
    }
}
