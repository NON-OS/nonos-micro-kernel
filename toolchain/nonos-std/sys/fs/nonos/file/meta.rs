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

// Metadata and the operations that have no VFS by-fd form. file_attr and
// size stat the path captured at open; duplicate is unsupported (the
// protocol has no dup); permission and time setters are accepted no-ops,
// matching the store's flat model.

use super::File;
use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::attr::{FileAttr, FilePermissions, FileTimes};
use crate::sys::fs::nonos::ops::stat;
use crate::sys::fs::nonos::transport::err;
use crate::sys::unsupported;

impl File {
    pub fn file_attr(&self) -> io::Result<FileAttr> {
        stat(Path::new(crate::str::from_utf8(&self.path).map_err(|_| err("bad path"))?))
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        Some(self.file_attr().map(|a| a.size()))
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        Ok(())
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        Ok(())
    }
}
