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

//! How long the serve loop waits between looks at the network.

use core::sync::atomic::{AtomicU32, Ordering};

/// What to wait while a connection is live. Short enough that a round trip
/// is not dominated by the wait, long enough that the loop still parks
/// rather than spinning a core.
const BUSY_MS: u64 = 2;

/// What to wait when nothing is asking. A machine with no connection open
/// should not be polling a card it has no reason to read.
const QUIET_MS: u64 = 50;

/// How many quiet turns to stay attentive after the last one that mattered.
///
/// A handshake is a sequence of short exchanges with real gaps between
/// them, and dropping straight back to the slow wait after each one would
/// pay the full cost on the next. Holding briefly covers the gap.
const LINGER: u32 = 64;

/// How long to wait for the next request.
pub fn next_wait(busy: &AtomicU32) -> u64 {
    if busy.load(Ordering::Relaxed) > 0 {
        BUSY_MS
    } else {
        QUIET_MS
    }
}

/// Something arrived, so stay attentive.
pub fn note_work(busy: &AtomicU32) {
    busy.store(LINGER, Ordering::Relaxed);
}

/// Nothing arrived. Count down towards settling.
pub fn note_idle(busy: &AtomicU32) {
    let left = busy.load(Ordering::Relaxed);
    if left > 0 {
        busy.store(left - 1, Ordering::Relaxed);
    }
}
