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

// Recursive directory removal. The store has no native form, so walk it:
// remove every child (recursing into subdirectories) and then the
// now-empty directory.

use super::rmdir::rmdir;
use super::unlink::unlink;
use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::dir::readdir;

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    for entry in readdir(path)? {
        let entry = entry?;
        let child = path.join(&entry.name);
        if entry.is_dir {
            remove_dir_all(&child)?;
        } else {
            unlink(&child)?;
        }
    }
    rmdir(path)
}
