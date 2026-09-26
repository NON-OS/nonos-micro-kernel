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

//! Store operations that change the namespace rather than content.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::root::Key;

type Fail = &'static str;

pub fn mkdir(at: &Key) -> Result<(), Fail> {
    vfs::mkdir(mk_getpid(), at.as_bytes())
}

pub fn rmdir(at: &Key) -> Result<(), Fail> {
    vfs::rmdir(mk_getpid(), at.as_bytes(), false)
}

pub fn unlink(at: &Key) -> Result<(), Fail> {
    vfs::unlink(mk_getpid(), at.as_bytes())
}

pub fn rename(from: &Key, to: &Key) -> Result<(), Fail> {
    vfs::rename(mk_getpid(), from.as_bytes(), to.as_bytes())
}

pub fn chmod(at: &Key, mode: u16) -> Result<(), Fail> {
    vfs::chmod(mk_getpid(), at.as_bytes(), mode)
}
