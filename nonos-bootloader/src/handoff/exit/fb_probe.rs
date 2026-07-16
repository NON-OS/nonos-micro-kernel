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

//! Stage dots for the blind window between the last splash line and the
//! kernel's first instruction. The GOP protocol dies at ExitBootServices
//! and the serial port has no reader on a laptop, but raw writes to the
//! latched framebuffer keep working across GOP shutdown, EBS, and the
//! CR3 switch (the new PML4 identity-maps the framebuffer). One dot per
//! stage, top-right corner, so a photo of a frozen screen counts how far
//! the exit path got.

use crate::display::gop::{get_dimensions, get_stride, is_initialized};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

const DOT: u32 = 22;
const GAP: u32 = 10;
const MARGIN: u32 = 12;

static FB_PTR: AtomicU64 = AtomicU64::new(0);
static FB_STRIDE_PX: AtomicU32 = AtomicU32::new(0);
static FB_WIDTH: AtomicU32 = AtomicU32::new(0);

/// Latch the framebuffer before `shutdown_for_exit` clears the display
/// state; the dots painted afterwards use this copy.
pub fn latch() {
    if !is_initialized() {
        return;
    }
    let (w, _h) = get_dimensions();
    FB_PTR.store(crate::display::gop::state::FB_PTR.load(Ordering::Relaxed), Ordering::Relaxed);
    FB_STRIDE_PX.store(get_stride(), Ordering::Relaxed);
    FB_WIDTH.store(w, Ordering::Relaxed);
}

/// Paint stage dot `n` (0-based, right to left) in cyan.
pub fn dot(n: u32) {
    let ptr = FB_PTR.load(Ordering::Relaxed);
    let stride = FB_STRIDE_PX.load(Ordering::Relaxed);
    let width = FB_WIDTH.load(Ordering::Relaxed);
    if ptr == 0 || stride == 0 || width == 0 || stride > 16384 {
        return;
    }
    let span = (n + 1).saturating_mul(DOT + GAP) + MARGIN;
    if span > width {
        return;
    }
    let x0 = (width - span) as u64;
    let base = ptr as *mut u32;
    for y in MARGIN as u64..(MARGIN + DOT) as u64 {
        for x in 0..DOT as u64 {
            let off = y * stride as u64 + x0 + x;
            // SAFETY: latched geometry bounds the writes to one dot inside
            // the visible frame; the framebuffer stays identity-mapped
            // through EBS and the CR3 switch on this path.
            unsafe { core::ptr::write_volatile(base.add(off as usize), 0xFF00_E5FF) };
        }
    }
}
