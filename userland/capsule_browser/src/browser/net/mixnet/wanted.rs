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

use core::sync::atomic::{AtomicBool, Ordering};

/// Whether the reader wants their traffic to leave through the mixnet.
///
/// On by default: a browser that has an anonymous route available and takes
/// the direct one without being asked publishes the address the route exists
/// to hide.
///
/// This is a choice, not a fallback. A request that the mixnet cannot carry
/// still fails rather than quietly going direct, because reverting on failure
/// would leak exactly when the network is worst. What this adds is the reader
/// being able to say, deliberately and in advance, that this session is not
/// one they need hidden.
static WANTED: AtomicBool = AtomicBool::new(true);

pub fn wanted() -> bool {
    WANTED.load(Ordering::Relaxed)
}

pub fn set_wanted(on: bool) {
    WANTED.store(on, Ordering::Relaxed);
}
