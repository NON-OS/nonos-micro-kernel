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

//! Backing a span of a guest with pages.

use nonos_libc::peer::{mk_peer_map, PEER_PROT_EXEC, PEER_PROT_WRITE};

use super::handle::Guest;
use super::layout::STACK_TOP;
use super::mem::{span_within, MAX_SPAN};
use super::region::Region;

impl Guest {
    /// Pages covering `[addr, addr + len)`.
    pub fn map(&mut self, addr: u64, len: u64, write: bool, exec: bool) -> i64 {
        // Bounded by the top of the guest's area, which is the stack.
        let Some((start, span)) = span_within(addr, len, STACK_TOP) else {
            return -1;
        };
        let mut prot = 0;
        if write {
            prot |= PEER_PROT_WRITE;
        }
        if exec {
            prot |= PEER_PROT_EXEC;
        }
        let mut done = 0;
        while done < span {
            let take = (span - done).min(MAX_SPAN);
            let rc = mk_peer_map(self.pid, start + done, take, prot);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        /*
         * Remembered because fork copies a guest by walking what its
         * supervisor gave it.
         */
        self.regions.push(Region { at: start, len: span, write, exec });
        0
    }
}
