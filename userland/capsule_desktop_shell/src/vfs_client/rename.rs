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

//! Rename or move an entry: the body is our pid, then the length-prefixed old
//! and new absolute paths. Moving a file into a folder is just a rename to a
//! path under that folder.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::call::call;
use super::constants::OP_RENAME;
use super::path;

pub fn rename(old: &[u8], new: &[u8]) -> bool {
    if !path::is_valid(old) || !path::is_valid(new) {
        return false;
    }
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(6 + old.len() + new.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(old.len() as u8);
    body.extend_from_slice(old);
    body.push(new.len() as u8);
    body.extend_from_slice(new);
    let mut rx = vec![0u8; 64];
    call(OP_RENAME, &body, &mut rx).is_some()
}
