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

use crate::linux::abi::errno;
use crate::linux::file::pread64;
use crate::linux::guest::Guest;

use super::map_exec::proven;
use super::map_req::MapReq;
use super::prot::PROT_EXEC;
use super::prot_span::protect_span;

pub fn file(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    /*
     * Asked before a page is allocated, not after the bytes are in place: a
     * mapping that would be refused should cost the guest nothing, and a
     * half-filled span left behind by a late refusal is memory the guest still
     * holds and did not ask to keep.
     */
    if req.prot & PROT_EXEC != 0 && !proven(guest, req.fd) {
        return errno::fail(errno::EPERM);
    }
    if guest.map(at, span, true, false) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    let mut done = 0u64;
    while done < req.len {
        let n = pread64(guest, req.fd, at + done, req.len - done, req.off + done) as i64;
        if n < 0 {
            return errno::fail(errno::EACCES);
        }
        if n == 0 {
            /*
             * Short of the requested span: the rest of the mapping is the
             * zeroes the fresh frames already hold, which is what a segment's
             * bss is.
             */
            break;
        }
        done += n as u64;
    }
    if protect_span(guest, at, span, req.prot) < 0 {
        return errno::fail(errno::EACCES);
    }
    if req.fixed().is_none() {
        guest.mmap_next += span;
    }
    errno::ok(at)
}
