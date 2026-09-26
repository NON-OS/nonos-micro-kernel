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

//! Proving a file before any of its pages become executable.

use crate::linux::file::{key, store_read};
use crate::linux::guest::{Guest, Kind};

/// The same ceiling the exec path reads an image under.
const MAX_IMAGE: u32 = 64 << 20;

/// Whether `fd` names a file this machine has agreed to execute.
pub fn proven(guest: &Guest, fd: u64) -> bool {
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::File) else {
        return false;
    };
    /*
     * A descriptor's path was normalised when it was opened, so it
     * needs no resolving here, only confining.
     */
    let at = &entry.path;
    let Ok(bytes) = store_read(&key(at), MAX_IMAGE) else {
        return false;
    };
    crate::linux::attest::verify(at, &bytes).is_ok()
}
