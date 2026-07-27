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

// The platform `FileDesc`: the RAII owner the fd-aware sys types wrap, the
// same role the unix and hermit `FileDesc` play. It holds an `OwnedFd` whose
// drop releases the table slot, which is what actually closes the backing
// service handle.

use crate::fmt;
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::{AsInner, FromInner, IntoInner};

pub struct FileDesc(OwnedFd);

impl AsInner<OwnedFd> for FileDesc {
    #[inline]
    fn as_inner(&self) -> &OwnedFd {
        &self.0
    }
}

impl FromInner<OwnedFd> for FileDesc {
    fn from_inner(owned: OwnedFd) -> FileDesc {
        FileDesc(owned)
    }
}

impl IntoInner<OwnedFd> for FileDesc {
    fn into_inner(self) -> OwnedFd {
        self.0
    }
}

impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for FileDesc {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromRawFd for FileDesc {
    unsafe fn from_raw_fd(fd: RawFd) -> FileDesc {
        // SAFETY: forwarded to the caller's obligation that `fd` is open and
        // exclusively owned.
        FileDesc(unsafe { OwnedFd::from_raw_fd(fd) })
    }
}

impl IntoRawFd for FileDesc {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl fmt::Debug for FileDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileDesc").field("fd", &self.0.as_raw_fd()).finish()
    }
}
