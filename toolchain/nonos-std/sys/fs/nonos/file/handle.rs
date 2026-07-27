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

// An open file handle over the VFS. Holds the pool port, the process id the
// service keys handles by, and the returned fd; the path is kept for the
// by-path operations the protocol has no by-fd form of (fstat, truncate).
// The vfs handle is owned through a `FileDesc` whose descriptor-table drop
// issues the close, so `os::fd` can borrow or take the descriptor like it
// does a unix file fd. The operations live one to a file in the sibling
// modules.

use crate::fmt;
use crate::sys::fd::FileDesc;
use crate::vec::Vec;

pub struct File {
    pub(super) port: u32,
    pub(super) pid: u32,
    pub(super) fd: u32,
    // Kept for the by-path operations the protocol lacks by-fd forms of
    // (fstat, ftruncate). Renaming the file while it is open makes these
    // miss, the usual race a path-based fallback accepts.
    pub(super) path: Vec<u8>,
    pub(super) desc: FileDesc,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").field("fd", &self.fd).finish()
    }
}
