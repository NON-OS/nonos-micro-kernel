// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::state::{convert_color, is_initialized, FB_HEIGHT, FB_PTR, FB_STRIDE, FB_WIDTH};
use core::sync::atomic::Ordering;

#[inline]
pub fn put_pixel(x: u32, y: u32, color: u32) {
    if !is_initialized() {
        return;
    }
    let (w, h, s) = (
        FB_WIDTH.load(Ordering::Relaxed),
        FB_HEIGHT.load(Ordering::Relaxed),
        FB_STRIDE.load(Ordering::Relaxed),
    );
    if x >= w || y >= h {
        return;
    }
    let fb = FB_PTR.load(Ordering::Relaxed) as *mut u32;
    unsafe {
        fb.offset((y * s + x) as isize).write_volatile(convert_color(color));
    }
}

pub fn fill_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    if !is_initialized() {
        return;
    }
    let (fw, fh, fs) = (
        FB_WIDTH.load(Ordering::Relaxed),
        FB_HEIGHT.load(Ordering::Relaxed),
        FB_STRIDE.load(Ordering::Relaxed),
    );
    let fb = FB_PTR.load(Ordering::Relaxed) as *mut u32;
    let nc = convert_color(color);
    for dy in 0..h {
        let py = y + dy;
        if py >= fh {
            break;
        }
        for dx in 0..w {
            let px = x + dx;
            if px >= fw {
                break;
            }
            unsafe {
                fb.offset((py * fs + px) as isize).write_volatile(nc);
            }
        }
    }
}

pub fn hline(x: u32, y: u32, w: u32, color: u32) {
    fill_rect(x, y, w, 1, color);
}
pub fn vline(x: u32, y: u32, h: u32, color: u32) {
    fill_rect(x, y, 1, h, color);
}

pub fn draw_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    hline(x, y, w, color);
    hline(x, y + h - 1, w, color);
    vline(x, y, h, color);
    vline(x + w - 1, y, h, color);
}

pub fn clear_screen(color: u32) {
    if !is_initialized() {
        return;
    }
    fill_rect(0, 0, FB_WIDTH.load(Ordering::Relaxed), FB_HEIGHT.load(Ordering::Relaxed), color);
}
