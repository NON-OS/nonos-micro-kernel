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
use crate::display::gop::{get_dimensions, put_pixel};

pub fn fill_atmosphere() {
    let (w, h) = get_dimensions();
    for y in 0..h {
        for x in 0..w {
            put_pixel(x, y, bg_at(x, y, w, h));
        }
    }
}

// Repaint just a rectangle of the atmosphere, so log lines clear back to the
// background instead of a solid box.
pub fn clear_region(x: u32, y: u32, rw: u32, rh: u32) {
    let (w, h) = get_dimensions();
    for dy in 0..rh {
        let py = y + dy;
        if py >= h {
            break;
        }
        for dx in 0..rw {
            let px = x + dx;
            if px >= w {
                break;
            }
            put_pixel(px, py, bg_at(px, py, w, h));
        }
    }
}
