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

// NONOS std fs: files over the kernel VFS service (vfs_pool) via IPC.
// Wires open/read/write/close, seek/tell, stat, truncate, rename, unlink,
// mkdir/rmdir, and readdir to the VFS protocol. Symlinks, hard links,
// per-file times, and locks are unsupported (the VFS does not model
// them); those paths return an error or no-op.
//
// The wire protocol lives in `transport`; the std-facing types and
// functions are split by concern into `file`, `attr`, `dir`,
// `open_options`, and `ops`, and re-exported here.

mod attr;
mod dir;
mod file;
mod open_options;
mod ops;
mod transport;

pub use crate::sys::fs::common::Dir;

pub use attr::{FileAttr, FilePermissions, FileType, FileTimes};
pub use dir::{DirBuilder, DirEntry, ReadDir, readdir};
pub use file::File;
pub use open_options::OpenOptions;
pub use ops::{
    canonicalize, copy, exists, link, lstat, readlink, remove_dir_all, rename, rmdir, set_perm,
    set_times, set_times_nofollow, stat, symlink, unlink,
};
