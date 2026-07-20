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

// The descriptor plumbing `os::fd` reaches through: borrow the descriptor a
// `File` owns, and convert between `File` and `FileDesc` for the
// `From<OwnedFd>`/`From<File>` chains.

use super::File;
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
use crate::sys::fd::{Backing, FileDesc, get};
use crate::sys::{FromInner, IntoInner};
use crate::vec::Vec;

impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.desc.as_raw_fd()
    }
}

impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.desc.as_fd()
    }
}

impl FromInner<FileDesc> for File {
    // Reconstruction from a bare descriptor (the `From<OwnedFd>` path). The
    // routing fields come back from the table; the open path is not
    // recoverable, so the by-path fallback operations (fstat, ftruncate)
    // report on the empty path. A descriptor that does not name a vfs handle
    // yields a dead file whose operations the service rejects.
    fn from_inner(desc: FileDesc) -> File {
        match get(desc.as_raw_fd()) {
            Some(Backing::File { port, pid, handle }) => {
                File { port, pid, fd: handle, path: Vec::new(), desc }
            }
            _ => File { port: 0, pid: 0, fd: 0, path: Vec::new(), desc },
        }
    }
}

impl IntoInner<FileDesc> for File {
    fn into_inner(self) -> FileDesc {
        self.desc
    }
}
