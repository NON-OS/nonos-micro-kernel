// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::constants::COLOR_BACKGROUND;
use crate::display::gop::{get_dimensions, is_initialized, put_pixel};

const CORE: u32 = 0xFF06_201C;

pub fn draw_vignette() {
    if !is_initialized() {
        return;
    }
    let (w, h) = get_dimensions();
    let (cx, cy) = (w as i64 / 2, h as i64 * 38 / 100);
    let r2 = ((w as i64 * w as i64) / 3).max(1);
    for y in 0..h as i64 {
        for x in 0..w as i64 {
            let dx = x - cx;
            let dy = (y - cy) * 12 / 10;
            let t = (((dx * dx + dy * dy) * 256 / r2).min(256)) as u32;
            put_pixel(x as u32, y as u32, lerp(CORE, COLOR_BACKGROUND, t));
        }
    }
}

pub(crate) fn lerp(a: u32, b: u32, t: u32) -> u32 {
    let mut out = 0xFF00_0000;
    let mut s = 0u32;
    while s < 24 {
        let c = (((a >> s) & 0xFF) * (256 - t) + ((b >> s) & 0xFF) * t) / 256;
        out |= (c & 0xFF) << s;
        s += 8;
    }
    out
}
