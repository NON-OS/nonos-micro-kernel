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

// Rename: OP_RENAME carries the process id and both length-prefixed paths.

use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::transport::{OP_RENAME, call, getpid, path_bytes, vfs_port};
use crate::vec::Vec;

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    let port = vfs_port()?;
    let ob = path_bytes(old)?;
    let nb = path_bytes(new)?;
    let mut body = Vec::with_capacity(6 + ob.len() + nb.len());
    body.extend_from_slice(&getpid().to_le_bytes());
    body.push(ob.len() as u8);
    body.extend_from_slice(&ob);
    body.push(nb.len() as u8);
    body.extend_from_slice(&nb);
    call(port, OP_RENAME, &body, 0).map(|_| ())
}
