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

/// Cleared for good the first time a running capsule could not be recorded.
///
/// Killing the spawn instead would be the wrong trade: the capsule is already
/// installed by then, and tearing it down on a bookkeeping failure turns a
/// full table into a denial of service. Understating what is on the machine is
/// the one error a remote party cannot detect, so the registry stops claiming
/// to be complete and attestation refuses rather than lies.
static COMPLETE: AtomicBool = AtomicBool::new(true);

/// True while every running capsule is present in the registry.
pub fn registry_complete() -> bool {
    COMPLETE.load(Ordering::Acquire)
}

/// One-way. A registry that has ever missed an entry cannot become
/// trustworthy again without a reboot, and a reboot is cheap here.
pub(super) fn mark_incomplete() {
    COMPLETE.store(false, Ordering::Release);
}
