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

use crate::display::constants::{COLOR_ACCENT, COLOR_ACCENT_DIM};
use crate::display::font::draw_string_4x;
use crate::display::gop::{get_dimensions, is_initialized};

const WORD: &[u8] = b"N\xd8NOS";
const GLOW_OUTER: u32 = 0xFF0A3A30;

pub fn wordmark_width() -> u32 {
    WORD.len() as u32 * 8 * 4
}

pub fn draw_wordmark(_x: u32, y: u32) {
    if !is_initialized() {
        return;
    }
    let (w, _) = get_dimensions();
    let x = (w.saturating_sub(wordmark_width())) / 2;
    for (dx, dy) in [(2i32, 0i32), (-2, 0), (0, 2), (0, -2)] {
        draw_string_4x(offset(x, dx), offset(y, dy), WORD, GLOW_OUTER);
    }
    for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
        draw_string_4x(offset(x, dx), offset(y, dy), WORD, COLOR_ACCENT_DIM);
    }
    draw_string_4x(x, y, WORD, COLOR_ACCENT);
}

fn offset(v: u32, d: i32) -> u32 {
    if d < 0 {
        v.saturating_sub(d.unsigned_abs())
    } else {
        v + d as u32
    }
}
