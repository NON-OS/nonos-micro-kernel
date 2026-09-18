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
use crate::linux::guest::{page_up, Guest};

use super::map_file::file;
use super::map_req::MapReq;
use super::prot::{wx_refused, PROT_EXEC, PROT_WRITE};

const MAP_SHARED: u64 = 0x01;
const MAP_ANONYMOUS: u64 = 0x20;

pub fn mmap(guest: &mut Guest, req: MapReq) -> u64 {
    if req.len == 0 {
        return errno::fail(errno::EINVAL);
    }
    if wx_refused(req.prot) {
        return errno::fail(errno::EPERM);
    }
    let span = page_up(req.len);
    let at = req.fixed().unwrap_or(guest.mmap_next);
    if req.flags & MAP_ANONYMOUS != 0 {
        return anonymous(guest, &req, at, span);
    }
    if req.flags & MAP_SHARED != 0 {
        /*
         * Sharing a file between processes needs frames that two address
         * spaces both point at, which no peer call offers. Refused rather
         * than quietly downgraded to a private copy, which would lose a
         * writer's changes with nothing to show that it had.
         */
        return errno::fail(errno::ENOSYS);
    }
    file(guest, &req, at, span)
}

fn anonymous(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    let write = req.prot & PROT_WRITE != 0;
    let exec = req.prot & PROT_EXEC != 0;
    if guest.map(at, span, write, exec) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    if req.fixed().is_none() {
        guest.mmap_next += span;
    }
    errno::ok(at)
}
