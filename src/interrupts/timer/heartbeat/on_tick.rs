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

use core::sync::atomic::{AtomicU32, Ordering};

use super::consts::{PALETTE, SIZE, X0, Y0};
use super::input_activity_bars::input_activity_bars;

static PHASE: AtomicU32 = AtomicU32::new(0);

/// Advance one palette step every 10 ticks. At a healthy 100 Hz tick that
/// is ~10 color changes per second (obviously alive in a photo); if the
/// timer is miscalibrated slow it crawls; if delivery stops it freezes.
pub fn on_tick(ticks: u64) {
    if ticks % 10 != 0 {
        return;
    }
    input_activity_bars();
    let phase = PHASE.fetch_add(1, Ordering::Relaxed);
    let color = PALETTE[(phase as usize) % PALETTE.len()];
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return;
    };
    if X0 + SIZE > fb.width || Y0 + SIZE > fb.height {
        return;
    }
    let row_px = (fb.stride / 4) as u64;
    let base = (fb.base_va.as_u64() + fb.offset as u64) as *mut u32;
    for y in 0..SIZE as u64 {
        for x in 0..SIZE as u64 {
            let off = (Y0 as u64 + y) * row_px + X0 as u64 + x;
            // SAFETY: bounds checked against the live framebuffer geometry;
            // the ISR is the only writer of this fixed corner rectangle.
            unsafe { core::ptr::write_volatile(base.add(off as usize), color) };
        }
    }
}
