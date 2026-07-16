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

//! Framebuffer panic banner. The legacy VGA text banner lives at 0xB8000,
//! which no UEFI machine displays, so the panic path also paints the
//! failure onto the GOP framebuffer whenever the kernel mapped one.

use super::draw::{draw_char, fill_rect};
use crate::kernel_core::init::framebuffer::{framebuffer_state, KernelFramebuffer};

const BAND_HEIGHT: u32 = 104;
const BAND_COLOR: u32 = 0x009C_1420;
const TITLE_COLOR: u32 = 0x00FF_E6E6;
const TEXT_COLOR: u32 = 0x00FF_FFFF;
const HINT_COLOR: u32 = 0x00E8_C4C4;

/// Paint a red panic band across the top of the screen with the panic
/// location. Safe to call from the panic handler: no allocation, no
/// locks, every pixel write bounds-checked.
pub fn show(file: &str, line: u32) {
    let Some(fb) = framebuffer_state() else {
        return;
    };
    fill_rect(fb, 0, 0, fb.width, BAND_HEIGHT.min(fb.height), BAND_COLOR);
    draw_text(fb, 24, 20, b"KERNEL PANIC", TITLE_COLOR);
    let mut x = draw_text(fb, 24, 48, file.as_bytes(), TEXT_COLOR);
    x = draw_text(fb, x, 48, b":", TEXT_COLOR);
    let mut digits = [0u8; 10];
    draw_text(fb, x, 48, fmt_u32(line, &mut digits), TEXT_COLOR);
    draw_text(fb, 24, 76, b"details on the serial console", HINT_COLOR);
}

fn draw_text(fb: &KernelFramebuffer, x0: u32, y: u32, text: &[u8], color: u32) -> u32 {
    let mut x = x0;
    for &b in text {
        if x + 8 >= fb.width {
            break;
        }
        draw_char(fb, x, y, b, color);
        x += 8;
    }
    x
}

fn fmt_u32(mut v: u32, out: &mut [u8; 10]) -> &[u8] {
    let mut k = out.len();
    loop {
        k -= 1;
        out[k] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 || k == 0 {
            break;
        }
    }
    &out[k..]
}
