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

//! Reading and writing a guest's memory, and giving it more.
//!
//! Every one of these is a peer call the kernel refuses unless this
//! process created the guest, so a personality bug cannot reach a process
//! it does not own.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::peer::{mk_peer_map, mk_peer_read, mk_peer_write, PEER_PROT_EXEC, PEER_PROT_WRITE};

use super::handle::Guest;

pub const PAGE: u64 = 4096;

pub fn page_down(addr: u64) -> u64 {
    addr & !(PAGE - 1)
}

pub fn page_up(addr: u64) -> u64 {
    (addr + PAGE - 1) & !(PAGE - 1)
}

impl Guest {
    /*
     * Pages covering `[addr, addr + len)`. Write and execute are separate
     * because the kernel refuses a page that is both, and a loader that
     * asked for both would be refused on the first code segment. Bytes
     * still reach a read-only page: a peer copy goes through the guest's
     * frames and not through its mapping.
     */
    pub fn map(&self, addr: u64, len: u64, write: bool, exec: bool) -> i64 {
        let start = page_down(addr);
        let span = page_up(addr + len) - start;
        let mut prot = 0;
        if write {
            prot |= PEER_PROT_WRITE;
        }
        if exec {
            prot |= PEER_PROT_EXEC;
        }
        mk_peer_map(self.pid, start, span, prot)
    }

    pub fn write(&self, addr: u64, bytes: &[u8]) -> i64 {
        mk_peer_write(self.pid, addr, bytes)
    }

    pub fn read(&self, addr: u64, len: usize) -> Option<Vec<u8>> {
        let mut out = vec![0u8; len];
        if mk_peer_read(self.pid, addr, &mut out) < 0 {
            return None;
        }
        Some(out)
    }
}
