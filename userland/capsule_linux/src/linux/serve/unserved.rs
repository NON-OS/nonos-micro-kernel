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


//! Naming a call this capsule does not serve.

use crate::linux::abi::{errno, name};

/// Name what was asked for. A guest that dies on a missing call should
/// leave behind the name of the call it needed.
pub fn unserved(number: u64) -> u64 {
    let mut line = [0u8; 64];
    let head = b"[LINUX] unserved ";
    let tag = name::of(number);
    let n = head.len().min(line.len());
    line[..n].copy_from_slice(&head[..n]);
    let m = (n + tag.len()).min(line.len());
    line[n..m].copy_from_slice(&tag[..m - n]);
    let end = (m + 1).min(line.len());
    line[m..end].copy_from_slice(b"\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), end);
    errno::fail(errno::ENOSYS)
}
