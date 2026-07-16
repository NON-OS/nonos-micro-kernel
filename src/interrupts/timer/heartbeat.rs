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

//! Timer-interrupt heartbeat. A small square in the top-left corner whose
//! color advances one step per second of ticks, painted straight to the
//! framebuffer from the timer ISR. It bypasses the compositor entirely, so
//! it keeps blinking even under the desktop: if this square animates on a
//! machine with no serial console, the LAPIC timer interrupt is firing; if
//! it is frozen, no timer interrupt is being delivered and every
//! tick-driven wakeup (preemption, sleeping capsules, the clock) is dead.
//! Bring-up diagnostic, compiled only under NONOS_FBCONSOLE.

use core::sync::atomic::{AtomicU32, Ordering};

const X0: u32 = 6;
const Y0: u32 = 30;
const SIZE: u32 = 16;

static PHASE: AtomicU32 = AtomicU32::new(0);

// 1 kHz-ish palette so a photographed frame lands on a definite color.
const PALETTE: [u32; 6] = [
    0xFF00_E5FF, // cyan
    0xFF00_D060, // green
    0xFFFF_D000, // gold
    0xFFFF_6000, // orange
    0xFFE0_2040, // red
    0xFFB0_50FF, // violet
];

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

// A second, independent marker to the right of the timer heartbeat, advanced
// once per delivered device interrupt (keyboard, mouse, touchpad, any broker
// IRQ). Frozen = no device interrupt has ever arrived; changing when you
// press a key = the IRQ is being delivered and the problem is downstream in
// the driver. Painted from the broker dispatch, which is lock free and gs
// free, so this stays raw framebuffer writes with no allocation.
static IRQ_PHASE: AtomicU32 = AtomicU32::new(0);
const IRQ_X0: u32 = 30;

pub fn device_irq_blip() {
    paint_blip(IRQ_X0, IRQ_PHASE.fetch_add(1, Ordering::Relaxed));
}

static POST_PHASE: AtomicU32 = AtomicU32::new(0);
const POST_X0: u32 = 54;

// Advanced once per input event a driver hands the kernel via
// mk_input_event_post. Frozen while you press keys or drag the touchpad means no
// driver is decoding and posting events: the break is in the driver or its
// hardware access, upstream of delivery. Changing means a driver is producing
// events and any remaining problem is downstream.
pub fn input_post_blip() {
    paint_blip(POST_X0, POST_PHASE.fetch_add(1, Ordering::Relaxed));
}

static DRAIN_PHASE: AtomicU32 = AtomicU32::new(0);
const DRAIN_X0: u32 = 78;

// Advanced once per batch the input router drains from the kernel ring via
// mk_input_event_drain. If the post marker changes but this one stays frozen,
// the router is not consuming events and they never reach the compositor/cursor.
pub fn input_drain_blip() {
    paint_blip(DRAIN_X0, DRAIN_PHASE.fetch_add(1, Ordering::Relaxed));
}

fn paint_blip(x0: u32, phase: u32) {
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

const BAR_X0: u32 = 6;
const BAR_H: u32 = 6;
const BAR_MAX: u32 = 600;

// Two persistent bars under the corner markers whose lengths grow with the count
// of input events the kernel has posted (green) and drained (blue). If the green
// bar lengthens when you touch the pad or press a key, a driver is producing
// events and the break is downstream; if it stays a stub, nothing reaches the
// kernel and the break is in the driver or its hardware access.
pub fn input_activity_bars() {
    use crate::kernel_core::surface_registry::{input_diag, input_drains, input_seq};
    // Stacked from top: each shows how far a driver got.
    //  white  = input drivers that reached their serving loop (~80px each)
    //  orange = i2c-HID touchpad found on its controller (~80px)
    //  purple = i8042 produced at least one byte (grows per byte)
    //  green  = input events posted to the kernel
    //  blue   = input events the router drained
    //  yellow = which I2C controller the host driver bound, 40px per index+1
    //           (I2C0 = 40px, I2C3 = 160px, I2C5 = 240px)
    //  red    = the bind was probe-confirmed by the touchpad's ACK (~80px);
    //           yellow without red = fallback bind, the pad never answered
    //           during setup
    draw_bar(40, (input_diag(1) as u32).saturating_mul(80).min(BAR_MAX).max(2), 0xFFF0_F0F0);
    draw_bar(48, (input_diag(2) as u32).saturating_mul(80).min(BAR_MAX).max(2), 0xFFFF_A000);
    draw_bar(56, (input_diag(3) as u32).min(BAR_MAX).max(2), 0xFFFF_40FF);
    draw_bar(64, (input_seq() as u32).min(BAR_MAX).max(2), 0xFF00_E000);
    draw_bar(72, (input_drains() as u32).min(BAR_MAX).max(2), 0xFF20_A0FF);
    //  cyan   = raw non-empty reports read from the touchpad, before decode
    //           (grows per report; flat while dragging = the pad sends nothing,
    //           growing while the green events bar stays flat = decode drops)
    //  lime   = the pad acknowledged RESET (wake handshake confirmed, ~80px)
    draw_bar(80, (input_diag(4) as u32).saturating_mul(40).min(BAR_MAX).max(2), 0xFFFF_E000);
    draw_bar(88, (input_diag(5) as u32).saturating_mul(80).min(BAR_MAX).max(2), 0xFFFF_2020);
    draw_bar(96, (input_diag(6) as u32).min(BAR_MAX).max(2), 0xFF00_E0E0);
    draw_bar(104, (input_diag(7) as u32).saturating_mul(80).min(BAR_MAX).max(2), 0xFF80_FF40);
    //  pink   = the ps2 driver read a port at least once (~80px): the PIO grant
    //           is live and the kernel can reach 0x60/0x64 for it. Pink present
    //           with the purple byte bar flat while you type = the i8042 is
    //           silent (kbd port disabled), not the grant. Pink absent = the
    //           driver cannot read the ports at all.
    draw_bar(112, (input_diag(0) as u32).saturating_mul(80).min(BAR_MAX).max(2), 0xFFFF_80C0);
}

fn draw_bar(y0: u32, len: u32, color: u32) {
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
