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

use crate::syscall::{call_raw, N_MK_SPAWN_INSTANCE};

/// Ask the kernel to open another window instance of an embedded app capsule
/// named by `name` (currently "app.terminal" or "app.browser"). The kernel
/// queues the request and init performs the attested spawn in its own context,
/// so this returns 0 once the request is accepted and the window appears a tick
/// later, or a negative errno: -2 unknown app, -16 the request queue is full,
/// -22 bad name. Gated on the SpawnWindow capability, so only the desktop shell
/// may call it.
pub fn mk_spawn_instance(name: &[u8]) -> i64 {
    call_raw(N_MK_SPAWN_INSTANCE, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}
