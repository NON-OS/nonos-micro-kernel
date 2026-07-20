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

// Error mapping for the VFS backend: a plain local error, and the
// translation of the service's negative unix errnos into the ErrorKind
// std callers match on (fs::exists, create_new, and friends).

use crate::io::{Error, ErrorKind};

pub(crate) fn err(msg: &'static str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

pub(crate) fn vfs_err(status: i32) -> Error {
    let kind = match status {
        -2 => ErrorKind::NotFound,
        -13 => ErrorKind::PermissionDenied,
        -17 => ErrorKind::AlreadyExists,
        -21 => ErrorKind::IsADirectory,
        -22 => ErrorKind::InvalidInput,
        -28 => ErrorKind::StorageFull,
        -39 => ErrorKind::DirectoryNotEmpty,
        _ => ErrorKind::Other,
    };
    Error::new(kind, "vfs op rejected")
}
