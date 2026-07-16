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

// Canonicalize a path. The store models no symlinks, so the canonical form
// is the absolute path with `.` and `..` folded away; it must name something
// that exists, so the folded path is stat'd before it is returned.

use crate::io;
use crate::path::{Component, Path, PathBuf};
use crate::sys::fs::nonos::ops::stat;
use crate::sys::fs::nonos::transport::cwd_bytes;

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    let abs: PathBuf = if p.is_absolute() {
        p.to_path_buf()
    } else {
        let cwd = cwd_bytes();
        let base = match crate::str::from_utf8(&cwd) {
            Ok(s) if !s.is_empty() => PathBuf::from(s),
            _ => PathBuf::from("/"),
        };
        base.join(p)
    };
    let mut out = PathBuf::from("/");
    for comp in abs.components() {
        match comp {
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(c) => out.push(c),
            _ => {}
        }
    }
    stat(&out)?;
    Ok(out)
}
