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

//! Every store operation this personality makes for a guest.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_getpid;

use super::root::Key;

type Fail = &'static str;

pub fn read(at: &Key, max: u32) -> Result<Vec<u8>, Fail> {
    vfs::read_file(mk_getpid(), at.as_bytes(), max)
}

pub fn write(at: &Key, data: &[u8]) -> Result<(), Fail> {
    vfs::write_file(mk_getpid(), at.as_bytes(), data)
}

pub fn stat(at: &Key) -> Result<(u64, bool), Fail> {
    vfs::stat(mk_getpid(), at.as_bytes())
}

pub fn stat_full(at: &Key) -> Result<(u64, bool, u64, bool), Fail> {
    vfs::stat_full(mk_getpid(), at.as_bytes())
}

pub fn list(at: &Key) -> Result<Vec<String>, Fail> {
    vfs::list_paths(mk_getpid(), at.as_bytes())
}

pub fn open(at: &Key) -> Result<VfsStream, Fail> {
    VfsStream::open(mk_getpid(), at.as_bytes())
}
