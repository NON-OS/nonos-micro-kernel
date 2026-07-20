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

use alloc::string::String;
use alloc::vec::Vec;

pub(super) const MAX_FILES: usize = 2048;
pub(super) const MAX_OPEN_FDS: usize = 256;
pub(super) const MAX_FILE_BYTES: usize = 1 << 26;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    NotFound,
    BadFd,
    Full,
    AccessDenied,
    Exists,
    NotEmpty,
    IsDir,
    Inval,
}

// How a seek offset is anchored; the wire encoding stays in the protocol
// layer, the store only sees the resolved intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekWhence {
    Set,
    Cur,
    End,
}

pub(super) type StoreResult<T> = Result<T, StoreError>;

pub(super) struct File {
    pub(super) name: String,
    pub(super) data: Vec<u8>,
    pub(super) is_dir: bool,
    // Last-modified wall-clock time in unix milliseconds, stamped at creation
    // and refreshed on every write.
    pub(super) mtime: u64,
    // Unix-style permission bits; only the owner-write bit (0o200) is enforced.
    pub(super) mode: u16,
}

// Default permissions for a new file and a new directory.
pub(super) const MODE_FILE: u16 = 0o644;
pub(super) const MODE_DIR: u16 = 0o755;
// Owner-write bit; its absence makes a file read-only.
pub(super) const MODE_WRITE: u16 = 0o200;

impl File {
    // Build a file stamped with the current time and default permissions.
    pub(super) fn new(name: String, data: Vec<u8>, is_dir: bool) -> Self {
        let mode = if is_dir { MODE_DIR } else { MODE_FILE };
        File { name, data, is_dir, mtime: super::time::now_ms(), mode }
    }
}

pub(super) struct OpenFd {
    pub(super) file_idx: usize,
    pub(super) owner_pid: u32,
    pub(super) pos: usize,
    pub(super) append: bool,
    pub(super) writable: bool,
}

pub struct Store {
    pub(super) files: Vec<File>,
    pub(super) fds: Vec<Option<OpenFd>>,
}
