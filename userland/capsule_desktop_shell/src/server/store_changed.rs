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

//! Whether the store has changed since the last time anyone looked.
//!
//! The desktop icons and the package list both need to notice a new file, and
//! both used to find out by listing a directory on every clock tick. That is a
//! tree walk, a serialisation and an IPC copy once a second for the life of the
//! session, to learn nothing almost every time.
//!
//! One counter read answers for both. It is shared rather than one per caller:
//! they run in the same pass, so a second read would return the same number and
//! cost another round trip to learn it.

use core::sync::atomic::{AtomicU64, Ordering};

use super::backoff::Backoff;

/// Last generation acted on. Starts at a value the store cannot report, so the
/// first pass always lists: a desktop that waited for a change before drawing
/// anything would come up empty.
const NEVER: u64 = u64::MAX;
static SEEN: AtomicU64 = AtomicU64::new(NEVER);

/// vfs_pool is unreachable while it stages packages, and every attempt through
/// that window costs a full timeout inside the shell's loop.
static GAP: Backoff = Backoff::new(250, 4000);

/// True when the store may have changed and it is worth paying for a listing.
///
/// A service that will not answer reports false, not true. Reporting true was
/// the bug: it sent the caller on to two directory listings that were about to
/// time out for the same reason the counter read just had, so one dead call per
/// tick became three. The desktop holds what it has and asks again later, which
/// is the same thing it would have shown anyway.
pub fn since_last_look() -> bool {
    if !GAP.due() {
        return false;
    }
    let Some(now) = crate::vfs_client::generation() else {
        GAP.missed();
        return false;
    };
    GAP.answered();
    SEEN.swap(now, Ordering::Relaxed) != now
}
