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

use nonos_surface::{create, share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

pub(super) fn register_share(
    base: *mut u32,
    width: u32,
    height: u32,
    stride: u32,
    byte_len: u64,
) -> Option<u64> {
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va: base as u64,
        flags: 0,
    };
    let sid = create(&desc);
    if sid < 0 {
        return None;
    }
    let handle = share(sid as u64);
    if handle <= 0 {
        return None;
    }
    Some(handle as u64)
}
