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
//! The file-creation mask.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// What a shell starts with, and what Linux gives a fresh process.
pub const DEFAULT_UMASK: u16 = 0o022;

pub fn umask(guest: &mut Guest, want: u64) -> u64 {
    let previous = guest.umask;
    /*
     * Only the nine permission bits are a mask; the rest are not the
     * caller's to set and Linux discards them too.
     */
    guest.umask = (want as u16) & 0o777;
    errno::ok(u64::from(previous))
}
