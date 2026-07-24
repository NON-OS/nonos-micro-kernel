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

extern crate alloc;
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::mk_getpid;

pub const MAX_FILE: u32 = 32 * 1024 * 1024;

pub fn load(path: &[u8]) -> Result<Vec<u8>, &'static str> {
    read_file(mk_getpid(), path, MAX_FILE)
}
