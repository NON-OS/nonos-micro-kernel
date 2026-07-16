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

// By-path metadata setters. The store has a flat model with no permission
// bits or per-file times to change, so setting permissions or times (with or
// without following a link, of which there are none) is an accepted no-op.

use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::attr::{FilePermissions, FileTimes};

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    Ok(())
}

pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    Ok(())
}

pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    Ok(())
}
