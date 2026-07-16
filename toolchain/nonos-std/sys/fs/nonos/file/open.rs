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

// File::open: resolve the pool port, approximate create-exclusive with a
// pre-check (the protocol has no atomic form), and issue OP_OPEN with the
// translated open flags.

use super::File;
use crate::io::{self, Error, ErrorKind};
use crate::path::Path;
use crate::sys::fs::nonos::open_options::OpenOptions;
use crate::sys::fs::nonos::ops::stat;
use crate::sys::fs::nonos::transport::{
    BODY_OFF, O_APPEND, O_CREATE, O_TRUNC, OP_OPEN, call, getpid, path_bytes, read_u32, vfs_port,
};
use crate::vec::Vec;

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let port = vfs_port()?;
        let pid = getpid();
        // The protocol has no atomic create-exclusive; a pre-check is the
        // honest approximation and reports the common case correctly.
        if opts.create_new && stat(path).is_ok() {
            return Err(Error::new(ErrorKind::AlreadyExists, "path exists"));
        }
        let mut flags = 0u32;
        if opts.create || opts.create_new {
            flags |= O_CREATE;
        }
        if opts.truncate {
            flags |= O_TRUNC;
        }
        if opts.append {
            flags |= O_APPEND;
        }
        let pb = path_bytes(path)?;
        let mut body = Vec::with_capacity(9 + pb.len());
        body.extend_from_slice(&pid.to_le_bytes());
        body.push(pb.len() as u8);
        body.extend_from_slice(&pb);
        body.extend_from_slice(&flags.to_le_bytes());
        let rx = call(port, OP_OPEN, &body, 8)?;
        Ok(File { port, pid, fd: read_u32(&rx, BODY_OFF), path: pb })
    }
}
