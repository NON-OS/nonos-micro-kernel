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

use crate::kernel_core::surface_registry::attach_map::state::{AttachRecord, ATTACHES};
use crate::kernel_core::surface_registry::types::SurfaceHandle;

pub fn record(pid: u32, handle: SurfaceHandle, base_va: u64, byte_len: u64) {
    let mut v = ATTACHES.lock();
    for rec in v.iter_mut() {
        if rec.pid == pid && rec.handle == handle {
            rec.base_va = base_va;
            rec.byte_len = byte_len;
            return;
        }
    }
    v.push(AttachRecord { pid, handle, base_va, byte_len });
}
