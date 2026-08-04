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

use spin::Mutex;

use crate::crypto::random::fill_random;

/// The key our own acknowledgements are recognised by.
///
/// One per client rather than one per message. An ack comes back with only an
/// encrypted fragment id in it, and this is what turns that back into a
/// fragment we sent; a key per message would mean holding one per packet in
/// flight to tell any of them apart.
static ACK_KEY: Mutex<Option<[u8; 16]>> = Mutex::new(None);

/// The ack key, drawn on first use.
///
/// Returns nothing when there is no entropy to draw one from. An ack keyed
/// with zeros would be recognisable as ours by anyone who guessed that, which
/// is the one thing it must not be.
pub fn ack_key() -> Option<[u8; 16]> {
    let mut slot = ACK_KEY.lock();
    if let Some(key) = *slot {
        return Some(key);
    }
    let mut key = [0u8; 16];
    fill_random(&mut key).ok()?;
    *slot = Some(key);
    Some(key)
}
