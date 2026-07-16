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

// stat and its thin relatives. OP_STAT returns size, a flags word (bit 0 is
// directory), and an appended mtime older replies omit. lstat has no symlinks
// to differ on, and exists is a stat that maps any error to "no".

use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::attr::FileAttr;
use crate::sys::fs::nonos::transport::{BODY_OFF, OP_STAT, call, err, getpid, path_body, read_u32, vfs_port};

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let port = vfs_port()?;
    let rx = call(port, OP_STAT, &path_body(getpid(), p)?, 24)?;
    let b = &rx[BODY_OFF..];
    if b.len() < 12 {
        return Err(err("short stat"));
    }
    let size = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
    let flags = read_u32(b, 8);
    // mtime is appended after size+flags; older replies omit it, so default to 0.
    let mtime = if b.len() >= 20 {
        u64::from_le_bytes([b[12], b[13], b[14], b[15], b[16], b[17], b[18], b[19]])
    } else {
        0
    };
    Ok(FileAttr::new(size, flags & 1 != 0, mtime))
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
}

pub fn exists(p: &Path) -> io::Result<bool> {
    Ok(stat(p).is_ok())
}
