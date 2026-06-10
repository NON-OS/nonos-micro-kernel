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

use super::font::get_char_bitmap;
use crate::kernel_core::init::framebuffer::KernelFramebuffer;

fn to_native(color: u32, bgr: bool) -> u32 {
    let c = 0xFF00_0000 | (color & 0x00FF_FFFF);
    if bgr {
        c
    } else {
        let r = (c >> 16) & 0xFF;
        let b = c & 0xFF;
        (c & 0xFF00_FF00) | (b << 16) | r
    }
}

pub(super) fn put_pixel(fb: &KernelFramebuffer, x: u32, y: u32, native: u32) {
    if x >= fb.width || y >= fb.height {
        return;
    }
    let off = (y as usize) * (fb.stride as usize) + (x as usize) * 4;
    let addr = fb.base_va.as_u64() + fb.offset as u64 + off as u64;
    unsafe { core::ptr::write_volatile(addr as *mut u32, native) }
}

pub(super) fn fill_rect(fb: &KernelFramebuffer, x: u32, y: u32, w: u32, h: u32, color: u32) {
    let native = to_native(color, fb.bgr);
    for dy in 0..h {
        for dx in 0..w {
            put_pixel(fb, x + dx, y + dy, native);
        }
    }
}

pub(super) fn draw_char(fb: &KernelFramebuffer, x: u32, y: u32, ch: u8, color: u32) {
    let native = to_native(color, fb.bgr);
    let glyph = get_char_bitmap(ch);
    for (row, &bits) in glyph.iter().enumerate() {
        for col in 0..8u32 {
            if (bits >> (7 - col)) & 1 == 1 {
                put_pixel(fb, x + col, y + row as u32, native);
            }
        }
    }
}
