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

use nonos_libc::{mk_mmap, nonos_display_dimensions};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub struct Overlay {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub byte_len: u64,
    pub backing_va: u64,
}

pub fn allocate() -> Result<Overlay, &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let rc = nonos_display_dimensions(0, &mut width as *mut u32, &mut height as *mut u32);
    if rc != 0 || width == 0 || height == 0 {
        return Err("display dimensions unavailable");
    }
    let stride = width.checked_mul(4).ok_or("stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("overlay size overflow")?;
    let base = mk_mmap(
        core::ptr::null_mut(),
        byte_len as usize,
        PROT_READ_WRITE,
        MAP_PRIVATE_ANON,
        -1,
        0,
    );
    if base.is_null() {
        return Err("overlay mmap failed");
    }
    Ok(Overlay { width, height, stride, byte_len, backing_va: base as u64 })
}
