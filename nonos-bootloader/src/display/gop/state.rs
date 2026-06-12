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

// The linear framebuffer the display module latched and the splash drew
// to, returned as (ptr, width, height, stride_bytes, bgr). FB_STRIDE is
// kept in pixels here, so it is converted to bytes for the handoff and
// the kernel, which both expect a byte stride. The MMIO address stays
// valid across ExitBootServices, so the handoff prefers this proven
// framebuffer over re-querying GOP, which is fragile after mode setup.
pub fn latched_linear_fb() -> Option<(u64, u32, u32, u32, bool)> {
    let ptr = FB_PTR.load(Ordering::Acquire);
    let width = FB_WIDTH.load(Ordering::Acquire);
    let height = FB_HEIGHT.load(Ordering::Acquire);
    let stride_px = FB_STRIDE.load(Ordering::Acquire);
    if ptr == 0 || width == 0 || height == 0 || stride_px < width {
        return None;
    }
    let stride_bytes = stride_px.saturating_mul(4);
    Some((ptr, width, height, stride_bytes, FB_FORMAT_BGR.load(Ordering::Acquire)))
}

pub fn shutdown_for_exit() {
    FB_INITIALIZED.store(false, Ordering::SeqCst);
}
