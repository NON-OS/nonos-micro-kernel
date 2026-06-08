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

use nonos_libc::{
    mk_mmap, mk_surface_register, mk_surface_share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub(crate) fn setup(w: u32, h: u32, stride: u32) -> Option<(u64, u64)> {
    let byte_len = (stride as u64).checked_mul(h as u64)?;
    let base = mk_mmap(
        core::ptr::null_mut(),
        byte_len as usize,
        PROT_READ_WRITE,
        MAP_PRIVATE_ANON,
        -1,
        0,
    );
    if base.is_null() {
        return None;
    }
    let base_va = base as u64;
    let desc = SurfaceDescriptor {
        width: w,
        height: h,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return None;
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        return None;
    }
    Some((base_va, handle as u64))
}
