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

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

pub static FB_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub static FB_PTR: AtomicU64 = AtomicU64::new(0);
pub static FB_WIDTH: AtomicU32 = AtomicU32::new(0);
pub static FB_HEIGHT: AtomicU32 = AtomicU32::new(0);
pub static FB_STRIDE: AtomicU32 = AtomicU32::new(0);
pub static FB_FORMAT_BGR: AtomicBool = AtomicBool::new(true);

#[inline]
pub fn is_initialized() -> bool {
    FB_INITIALIZED.load(Ordering::Relaxed)
}
#[inline]
pub fn get_dimensions() -> (u32, u32) {
    (FB_WIDTH.load(Ordering::Relaxed), FB_HEIGHT.load(Ordering::Relaxed))
}
#[inline]
pub fn get_stride() -> u32 {
    FB_STRIDE.load(Ordering::Relaxed)
}

#[inline]
pub fn convert_color(argb: u32) -> u32 {
    if FB_FORMAT_BGR.load(Ordering::Relaxed) {
        argb
    } else {
        let (a, r, g, b) =
            ((argb >> 24) & 0xFF, (argb >> 16) & 0xFF, (argb >> 8) & 0xFF, argb & 0xFF);
        (a << 24) | (b << 16) | (g << 8) | r
    }
}

pub fn shutdown_for_exit() {
    FB_INITIALIZED.store(false, Ordering::SeqCst);
}
