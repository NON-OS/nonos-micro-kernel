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

use super::consts::{BAR_H, BAR_X0};

pub(super) fn draw_bar(y0: u32, len: u32, color: u32) {
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return;
    };
    if y0 + BAR_H > fb.height || BAR_X0 >= fb.width {
        return;
    }
    let w = len.min(fb.width - BAR_X0);
    let row_px = (fb.stride / 4) as u64;
    let base = (fb.base_va.as_u64() + fb.offset as u64) as *mut u32;
    for y in 0..BAR_H as u64 {
        for x in 0..w as u64 {
            let off = (y0 as u64 + y) * row_px + BAR_X0 as u64 + x;
            // SAFETY: bounds checked against the live framebuffer geometry; this
            // fixed bar region has a single writer (the timer tick).
            unsafe { core::ptr::write_volatile(base.add(off as usize), color) };
        }
    }
}
