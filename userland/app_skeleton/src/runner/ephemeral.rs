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

//! Whether this capsule is an on-demand window instance. The kernel tags such
//! spawns with a marker in argv; an instance must exit when its window closes
//! so its RAM is zeroized and its slot freed, instead of lingering idle. A
//! base app carries no marker and returns to idle on close as before.

use alloc::vec;

use nonos_libc::mk_args;

// Must match `INSTANCE_ARG` on the kernel side (capsule_spawn::instance).
const INSTANCE_ARG: &[u8] = b"--nonos-window-instance";

/// True if this capsule was spawned as an on-demand window instance.
pub(super) fn is_window_instance() -> bool {
    let needed = mk_args(core::ptr::null_mut(), 0);
    if needed <= 0 {
        return false;
    }
    let mut buf = vec![0u8; needed as usize];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return false;
    }
    // argv arrives NUL separated; the marker is one whole argument.
    buf[..n as usize].split(|&b| b == 0).any(|arg| arg == INSTANCE_ARG)
}
