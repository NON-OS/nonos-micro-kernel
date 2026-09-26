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

//! Moving a name.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::resolve::key;
use super::store_name;

pub fn rename(guest: &Guest, old: u64, new: u64) -> u64 {
    let (Some(from), Some(to)) = (
        resolve_at(guest, super::flags::AT_FDCWD, old),
        resolve_at(guest, super::flags::AT_FDCWD, new),
    ) else {
        return errno::fail(errno::EFAULT);
    };
    match store_name::rename(&key(&from), &key(&to)) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}
