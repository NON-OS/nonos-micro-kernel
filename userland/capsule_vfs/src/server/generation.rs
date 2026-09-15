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

//! A counter that changes when the store does.
//!
//! The desktop and the package list each ran a full directory listing on every
//! clock tick, forever, so the shell asked vfs_pool to walk and serialise a
//! directory once a second for the life of the session to notice a file that
//! almost never appears. The answer was thrown away unchanged nearly every
//! time.
//!
//! The honest fix is a notification, and that means subscriber state and
//! outbound sends inside a service every capsule depends on. This is the cheap
//! two thirds of it: a number that moves when anything mutates, so a caller
//! reads eight bytes to learn there is nothing to do, and pays for a listing
//! only when there is.
//!
//! Bumped on the attempt rather than on success. A failed mkdir moving the
//! counter costs one redundant listing; a handler forgetting to bump it leaves
//! a desktop that never updates, and those two mistakes are not the same size.

use core::sync::atomic::{AtomicU64, Ordering};

static GENERATION: AtomicU64 = AtomicU64::new(0);

/// Record that something in the store may have changed.
pub fn bump() {
    GENERATION.fetch_add(1, Ordering::Relaxed);
}

/// The current value. Relaxed is right: a caller that reads a value one behind
/// simply asks again on its next tick, and there is no other state being
/// published alongside it that the ordering would have to protect.
pub fn current() -> u64 {
    GENERATION.load(Ordering::Relaxed)
}
