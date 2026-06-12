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

use super::draw::{draw_char, fill_rect};
use super::state::{DISPLAY_ENABLED, LOG_Y, MIN_LOG_Y};
use crate::kernel_core::init::framebuffer::framebuffer_state;
use core::sync::atomic::Ordering;

const LINE_H: u32 = 18;
const MSG_COLOR: u32 = 0x00C8D0D8;

pub(super) fn write_line(tag: &str, msg: &str, color: u32) {
    if !DISPLAY_ENABLED.load(Ordering::Acquire) {
        return;
    }
    let Some(fb) = framebuffer_state() else {
        return;
    };
    let mut y = LOG_Y.load(Ordering::Relaxed);
    if y + LINE_H + 4 >= fb.height {
        let top = MIN_LOG_Y.load(Ordering::Relaxed);
        fill_rect(fb, 0, top, fb.width, fb.height - top, 0x0000_0000);
        y = top;
    }
    let mut x = 24u32;
    draw_char(fb, x, y, b'[', color);
    x += 8;
    for &b in tag.as_bytes() {
        draw_char(fb, x, y, b, color);
        x += 8;
    }
    draw_char(fb, x, y, b']', color);
    x += 16;
    for &b in msg.as_bytes() {
        if x + 8 >= fb.width {
            break;
        }
        draw_char(fb, x, y, b, MSG_COLOR);
        x += 8;
    }
    LOG_Y.store(y + LINE_H, Ordering::Relaxed);
}
