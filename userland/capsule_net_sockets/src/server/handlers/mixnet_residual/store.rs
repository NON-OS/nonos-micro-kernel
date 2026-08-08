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

use super::types::RESIDUAL;
use crate::sockets::SocketKey;

/// Hold what a read could not take.
///
/// A frame is gone from the mixnet queue once it has been read, so a caller
/// with a smaller buffer than the frame would lose the difference outright.
/// Keeping it means the next read continues the stream rather than skipping
/// part of it. Returns false when the remainder could not be held, which is
/// the one case where bytes are still dropped and so is worth telling apart
/// from a read that simply came up short.
pub fn store(key: SocketKey, rest: &[u8]) -> bool {
    if rest.is_empty() {
        return true;
    }
    let mut slots = RESIDUAL.lock();
    // A socket keeps whichever slot it already holds rather than claiming
    // another, or a reader that stays behind orphans one slot per frame and
    // works its way through the table.
    let at = match slots.iter().position(|s| s.pid == key.pid && s.handle == key.handle) {
        // Bytes still sitting there come earlier in the stream, so they
        // cannot be dropped to make room for later ones. A read drains the
        // slot before it pulls another frame, so this means a caller went
        // out of order.
        Some(at) if slots[at].off < slots[at].len => return false,
        Some(at) => at,
        None => match slots.iter().position(|s| s.off >= s.len) {
            Some(at) => at,
            None => return false,
        },
    };
    let slot = &mut slots[at];
    slot.pid = key.pid;
    slot.handle = key.handle;
    slot.buf[..rest.len()].copy_from_slice(rest);
    slot.len = rest.len();
    slot.off = 0;
    true
}
