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
    mk_mmap, mk_munmap, mk_surface_register, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};
use nonos_toolkit::types::ImageSize;

use crate::protocol::{E_INVAL, E_NOMEM};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub fn register_argb_surface(pixels: &[u32], size: ImageSize) -> Result<(u64, u32, u64), i32> {
    let stride = size.width.checked_mul(4).ok_or(E_INVAL)?;
    let byte_len = (stride as u64).checked_mul(size.height as u64).ok_or(E_INVAL)?;
    let base =
        mk_mmap(core::ptr::null_mut(), byte_len as usize, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if (base as isize) <= 0 {
        return Err(E_NOMEM);
    }
    let words = (byte_len / 4) as usize;
    let dst = base as *mut u32;
    unsafe { core::ptr::copy_nonoverlapping(pixels.as_ptr(), dst, words) };
    let desc = SurfaceDescriptor {
        width: size.width,
        height: size.height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va: base as u64,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        if mk_munmap(base, byte_len as usize) < 0 {
            return Err(E_NOMEM);
        }
        return Err(sid as i32);
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        return Err(handle as i32);
    }
    Ok((handle as u64, stride, byte_len))
}
