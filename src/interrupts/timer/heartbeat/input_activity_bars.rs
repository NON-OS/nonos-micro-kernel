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

use super::consts::BAR_MAX;
use super::draw_bar::draw_bar;

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
