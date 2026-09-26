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

//! The descriptor a program opens to reach its nameserver.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

/// It looks like a datagram socket and holds no handle: there is
/// nothing on the other side of it, which is the point.
pub fn open(guest: &mut Guest) -> u64 {
    match crate::linux::file::install(guest, Fd::resolver()) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
