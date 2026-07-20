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

// Metadata types the VFS backend hands to std: file size and type, the
// coarse readonly permission bit, and the modified-time stamp the store
// records. The store keeps no access or creation time, so those fall back
// to the modified time. Built through the pub(crate) constructors so the
// fields stay private to this file.

use crate::io;
use crate::sys::time::SystemTime;

#[derive(Clone)]
pub struct FileAttr {
    size: u64,
    is_dir: bool,
    // Last-modified time in unix milliseconds, stamped by the store. 0 means
    // the store had no clock when the file was created.
    mtime: u64,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    readonly: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType {
    is_dir: bool,
}

impl FileAttr {
    pub(crate) fn new(size: u64, is_dir: bool, mtime: u64) -> FileAttr {
        FileAttr { size, is_dir, mtime }
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn perm(&self) -> FilePermissions {
        FilePermissions { readonly: false }
    }
    pub fn file_type(&self) -> FileType {
        FileType { is_dir: self.is_dir }
    }
    pub fn modified(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::from_unix_millis(self.mtime))
    }
    pub fn accessed(&self) -> io::Result<SystemTime> {
        // The store tracks no access time; fall back to the modified time.
        Ok(SystemTime::from_unix_millis(self.mtime))
    }
    pub fn created(&self) -> io::Result<SystemTime> {
        // The store stamps mtime at creation, so it is the closest thing to a
        // creation time it has.
        Ok(SystemTime::from_unix_millis(self.mtime))
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.readonly
    }
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub(crate) fn new(is_dir: bool) -> FileType {
        FileType { is_dir }
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
    pub fn is_symlink(&self) -> bool {
        false
    }
}
