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

// Remove an empty directory: OP_RMDIR by path. The store rejects a
// non-empty directory with -39, which surfaces as DirectoryNotEmpty.

use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::transport::{OP_RMDIR, call, getpid, path_body, vfs_port};

pub fn rmdir(p: &Path) -> io::Result<()> {
    let port = vfs_port()?;
    call(port, OP_RMDIR, &path_body(getpid(), p)?, 0).map(|_| ())
}
