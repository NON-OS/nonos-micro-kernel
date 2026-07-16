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

//! Early-boot breadcrumb strip painted straight into the handoff
//! framebuffer. On machines with no serial console the strip is the only
//! record of how far the kernel got before the on-screen log exists: each
//! milestone paints one colored segment along the top of the panel, so a
//! photo of a frozen screen localizes the hang.

use crate::boot::handoff::BootHandoffV1;

const SEG_WIDTH: u32 = 180;
const SEG_GAP: u32 = 20;
const SEG_HEIGHT: u32 = 10;

/// Paint breadcrumb segment `n` by raw write to the framebuffer's physical
/// address. Only valid before the kernel installs its own page tables,
/// while the bootloader's mapping of the framebuffer is still live; after
/// that, milestones paint through the mapped framebuffer instead.
pub fn paint(handoff: &BootHandoffV1, n: u32, argb: u32) {
    let fb = &handoff.fb;
    // The caller may hand this the raw, unvalidated handoff, so every
    // geometry field is treated as hostile.
    if fb.width == 0 || fb.width > 8192 || fb.height == 0 || fb.height > 8192 {
        return;
    }
    let Some(row_bytes) = fb.width.checked_mul(4) else {
        return;
    };
    if fb.ptr == 0 || fb.stride < row_bytes || fb.stride > 65536 {
        return;
    }
    let x0 = n.saturating_mul(SEG_WIDTH + SEG_GAP);
    let Some(x_end) = x0.checked_add(SEG_WIDTH) else {
        return;
    };
    if x_end > fb.width || SEG_HEIGHT > fb.height {
        return;
    }
    let row_px = (fb.stride / 4) as u64;
    let base = fb.ptr as *mut u32;
    for y in 0..SEG_HEIGHT as u64 {
        for x in 0..SEG_WIDTH as u64 {
            let off = y * row_px + x0 as u64 + x;
            // SAFETY: bounds are checked against the handoff geometry above
            // and the bootloader's framebuffer mapping is still installed on
            // this early path; each pixel write stays inside one scanline.
            unsafe { core::ptr::write_volatile(base.add(off as usize), argb) };
        }
    }
}
