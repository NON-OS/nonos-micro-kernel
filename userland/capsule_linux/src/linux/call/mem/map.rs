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

//! `mmap`: anonymous pages, or a private mapping of a file.

use crate::linux::abi::errno;
use crate::linux::guest::{span_within, Guest, MMAP_LIMIT, STACK_TOP};

use super::map_anon::{anonymous, memfd};
use super::map_file::file;
use super::map_req::MapReq;
use super::prot::wx_refused;

const MAP_SHARED: u64 = 0x01;
const MAP_ANONYMOUS: u64 = 0x20;

pub fn mmap(guest: &mut Guest, req: MapReq) -> u64 {
    if req.len == 0 {
        return errno::fail(errno::EINVAL);
    }
    if wx_refused(req.prot) {
        return errno::fail(errno::EPERM);
    }
    // The ceiling differs by who chose the address.
    let (at, limit) = match req.fixed() {
        Some(addr) => (addr, STACK_TOP),
        None => (guest.mmap_next, MMAP_LIMIT),
    };
    let Some((at, span)) = span_within(at, req.len, limit) else {
        return errno::fail(errno::ENOMEM);
    };
    if req.flags & MAP_ANONYMOUS != 0 {
        return anonymous(guest, &req, at, span);
    }
    if crate::linux::file::is_memfd(guest, req.fd) {
        return memfd(guest, &req, at, span);
    }
    if req.flags & MAP_SHARED != 0 {
        /*
         * Sharing a file between processes needs frames that two address
         * spaces both point at, which no peer call offers.
         */
        return errno::fail(errno::ENOSYS);
    }
    file(guest, &req, at, span)
}
