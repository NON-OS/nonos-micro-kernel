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


//! A private file mapping.
//!
//! The pages are filled here rather than paged in on first touch: they
//! are mapped writable, the bytes are read into them, and the protection
//! the caller asked for is set afterwards. A dynamic linker needs exactly
//! that order, and the cost is honest, which is that the whole span is
//! read at once rather than on demand.

use crate::linux::abi::errno;
use crate::linux::file::pread64;
use crate::linux::guest::Guest;

use super::map_req::MapReq;
use super::prot::protect_span;

pub fn file(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    if guest.map(at, span, true, false) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    if (pread64(guest, req.fd, at, req.len, req.off) as i64) < 0 {
        return errno::fail(errno::EACCES);
    }
    if protect_span(guest, at, span, req.prot) < 0 {
        return errno::fail(errno::EACCES);
    }
    if req.fixed().is_none() {
        guest.mmap_next += span;
    }
    errno::ok(at)
}
