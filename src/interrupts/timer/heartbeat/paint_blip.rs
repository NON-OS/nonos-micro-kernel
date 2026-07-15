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

use super::consts::{PALETTE, SIZE, Y0};

pub(super) fn paint_blip(x0: u32, phase: u32) {
    let color = PALETTE[(phase as usize) % PALETTE.len()];
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return;
    };
    if x0 + SIZE > fb.width || Y0 + SIZE > fb.height {
        return;
    }
    let row_px = (fb.stride / 4) as u64;
    let base = (fb.base_va.as_u64() + fb.offset as u64) as *mut u32;
    for y in 0..SIZE as u64 {
        for x in 0..SIZE as u64 {
            let off = (Y0 as u64 + y) * row_px + x0 as u64 + x;
            // SAFETY: bounds checked against the live framebuffer geometry; each
            // fixed corner column has a single writer.
            unsafe { core::ptr::write_volatile(base.add(off as usize), color) };
        }
    }
}
