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

//! Giving a guest's pages back.

use nonos_libc::peer::mk_peer_unmap;

use super::handle::Guest;
use super::layout::STACK_TOP;
use super::mem::{span_within, MAX_SPAN};
use super::region_cut::cut;

impl Guest {
    /// Return `[addr, addr + len)` to the kernel.
    pub fn unmap(&mut self, addr: u64, len: u64) -> i64 {
        let Some((start, span)) = span_within(addr, len, STACK_TOP) else {
            return -1;
        };
        let mut done = 0;
        while done < span {
            let take = (span - done).min(MAX_SPAN);
            let rc = mk_peer_unmap(self.pid, start + done, take);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        self.regions = cut(&self.regions, start, span);
        0
    }
}
