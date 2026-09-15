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

//! Who may write the medium this driver guards.
//!
//! The package store at LBA 0 is read back as trusted input on the next
//! boot, so the write path answers the kernel-internal client, which arrives
//! as sender pid 0 because every real capsule's envelope is kernel-stamped,
//! and otherwise only a sender the kernel says holds StoreWrite. The kernel
//! is asked on every request: a cached verdict would outlive the holder's
//! exit and follow its pid to whatever process is handed that pid next.

mod rule;

use nonos_libc::mk_cap_check;

/// StoreWrite, as abi/caps.toml numbers it. Held by the installer and by the
/// vfs server, and by nothing that merely draws a window.
pub const CAP_STORE_WRITE: u64 = 1 << 26;

pub fn permits(op: u16, sender_pid: u32) -> bool {
    if rule::allows(op, sender_pid, false) {
        return true;
    }
    rule::allows(op, sender_pid, mk_cap_check(sender_pid, CAP_STORE_WRITE) == 1)
}
