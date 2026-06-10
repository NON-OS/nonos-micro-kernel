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

pub fn get_pixel(x: u32, y: u32) -> u32 {
    if !is_initialized() {
        return 0;
    }
    let (w, h, s) = (
        FB_WIDTH.load(Ordering::Relaxed),
        FB_HEIGHT.load(Ordering::Relaxed),
        FB_STRIDE.load(Ordering::Relaxed),
    );
    if x >= w || y >= h {
        return 0;
    }
    let fb = FB_PTR.load(Ordering::Relaxed) as *const u32;
    let native = unsafe { fb.offset((y * s + x) as isize).read_volatile() };
    convert_color(native)
}
