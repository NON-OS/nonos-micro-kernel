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

//! Mappings with no file behind them.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::map_req::MapReq;
use super::prot::{PROT_EXEC, PROT_WRITE};

/// A memfd has nothing to read in: it is pages, and the client is about to
/// draw into them.
pub fn memfd(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    let out = anonymous(guest, req, at, span);
    if (out as i64) < 0 {
        return out;
    }
    crate::linux::file::set_mapped(guest, req.fd, at);
    /*
     * A descriptor this capsule staged content on, the keymap being the one
     * that matters, has to arrive with those bytes already in it: the client
     * maps it and reads it without ever issuing a read.
     */
    if let Some(bytes) = crate::linux::file::staged(guest, req.fd) {
        if guest.write(at, &bytes) < bytes.len() as i64 {
            return errno::fail(errno::EFAULT);
        }
    }
    out
}

pub fn anonymous(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
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
