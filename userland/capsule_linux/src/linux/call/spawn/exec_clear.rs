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

//! Emptying a guest's address space.

use nonos_libc::peer::mk_peer_unmap;

use crate::linux::guest::{Guest, MAX_SPAN};

/// Every span this capsule gave the guest, taken back.
pub fn clear(guest: &mut Guest) {
    for span in core::mem::take(&mut guest.regions) {
        let mut done = 0;
        /*
         * One peer call per megabyte: the kernel refuses a longer span rather
         * than sitting in a loop with the page tables locked.
         */
        while done < span.len {
            let take = (span.len - done).min(MAX_SPAN);
            let _ = mk_peer_unmap(guest.pid, span.at + done, take);
            done += take;
        }
    }
}

/// Every descriptor the guest marked close-on-exec.
pub fn shed(guest: &mut Guest) {
    for fd in 0..guest.fds.len() as u64 {
        if guest.fds.get(fd as usize).is_some_and(|f| f.cloexec && f.is_open()) {
            let _ = crate::linux::file::close(guest, fd);
        }
    }
}
