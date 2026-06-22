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

use super::bg::bg_at;
use super::color::mix;
use crate::display::gop::{get_dimensions, put_pixel};

// Blend a translucent rect over the (known) atmosphere background, writing
// each pixel once. No framebuffer reads, so it stays cheap on real hardware.
pub fn blend_rect(x: u32, y: u32, w: u32, h: u32, color: u32, alpha: u32) {
    let (fw, fh) = get_dimensions();
    for dy in 0..h {
        let py = y + dy;
        if py >= fh {
            break;
        }
        for dx in 0..w {
            let px = x + dx;
            if px >= fw {
                break;
            }
            put_pixel(px, py, mix(bg_at(px, py, fw, fh), color, alpha));
        }
    }
}
