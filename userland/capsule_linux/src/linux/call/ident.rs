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
//! Who the guest is, and which process group it belongs to.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// The identity every guest runs as.
const GUEST_UID: u64 = 0;

pub fn getppid(guest: &Guest) -> u64 {
    // The personality is the parent of every guest it hosts.
    errno::ok(u64::from(guest.parent))
}


/// Setting the identity to the one already held is the only change
/// that can be honoured, so it is the only one accepted.
pub fn setuid(want: u64) -> u64 {
    match want {
        GUEST_UID => errno::ok(0),
        _ => errno::fail(errno::EPERM),
    }
}
