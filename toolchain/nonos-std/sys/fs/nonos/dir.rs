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

// Directory reads and creation over the VFS: readdir issues OP_LIST and
// unpacks the length-prefixed name list, DirBuilder issues OP_MKDIR. The
// store marks a directory by a trailing slash on its listed name.

use super::attr::{FileAttr, FileType};
use super::transport::{BODY_OFF, OP_LIST, OP_MKDIR, call, getpid, path_body, vfs_port};
use crate::ffi::OsString;
use crate::fmt;
use crate::io;
use crate::path::{Path, PathBuf};
use crate::vec::Vec;

pub struct ReadDir {
    entries: crate::vec::IntoIter<DirEntry>,
}

#[derive(Clone)]
pub struct DirEntry {
    pub(super) name: PathBuf,
    pub(super) is_dir: bool,
}

#[derive(Debug)]
pub struct DirBuilder {}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadDir").finish_non_exhaustive()
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.entries.next().map(Ok)
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.name.clone()
    }
    pub fn file_name(&self) -> OsString {
        self.name.as_os_str().to_os_string()
    }
    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(FileAttr::new(0, self.is_dir, 0))
    }
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType::new(self.is_dir))
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }
    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        let port = vfs_port()?;
        call(port, OP_MKDIR, &path_body(getpid(), p)?, 0).map(|_| ())
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    let port = vfs_port()?;
    let rx = call(port, OP_LIST, &path_body(getpid(), p)?, 65536)?;
    let mut out: Vec<DirEntry> = Vec::new();
    let mut body = &rx[BODY_OFF..];
    while !body.is_empty() {
        let len = body[0] as usize;
        if body.len() < 1 + len {
            break;
        }
        if let Ok(name) = crate::str::from_utf8(&body[1..1 + len]) {
            let is_dir = name.ends_with('/');
            let trimmed = name.trim_end_matches('/');
            out.push(DirEntry { name: PathBuf::from(trimmed), is_dir });
        }
        body = &body[1 + len..];
    }
    Ok(ReadDir { entries: out.into_iter() })
}
