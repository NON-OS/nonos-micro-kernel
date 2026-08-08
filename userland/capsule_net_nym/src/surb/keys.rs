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

use super::types::SURB_KEY_BYTES;

/// How many outstanding reply keys are kept.
///
/// A reply arrives sealed under one of these and carries nothing saying
/// which, so every one held is a candidate to try. Keeping them forever would
/// grow that work without bound; the oldest is dropped instead, which costs a
/// reply that came back long after the request it answers.
///
/// Every request hands out a fresh set, and a mixnet round trip is measured
/// in seconds, so several are always outstanding at once. Held too few and
/// the ring wraps while replies are still in the air: the key that opens one
/// is gone by the time it lands, and a real answer is dropped as though it
/// were addressed to somebody else. Sized for the tens of requests a page
/// load actually produces.
const CAP: usize = 512;

static KEYS: Mutex<[[u8; SURB_KEY_BYTES]; CAP]> = Mutex::new([[0u8; SURB_KEY_BYTES]; CAP]);
static NEXT: Mutex<usize> = Mutex::new(0);

/// Keep a key so a reply sealed under it can be opened.
pub fn remember(key: [u8; SURB_KEY_BYTES]) {
    let mut at = NEXT.lock();
    KEYS.lock()[*at % CAP] = key;
    *at = at.wrapping_add(1);
}

/// Every key a reply might be sealed under, newest first.
///
/// Newest first because a reply usually answers the most recent request, so
/// the first key tried is the likeliest to be the right one.
pub fn candidates() -> [[u8; SURB_KEY_BYTES]; CAP] {
    let held = *KEYS.lock();
    let at = *NEXT.lock();
    let mut out = [[0u8; SURB_KEY_BYTES]; CAP];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = held[(at + CAP - 1 - index) % CAP];
    }
    out
}
