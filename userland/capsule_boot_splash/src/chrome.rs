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

use nonos_toolkit::font::render::draw_text;

const ACCENT: u32 = 0xFF00_D4AA;
const BORDER: u32 = 0xFF13_4237;

pub(crate) fn rect(buf: &mut [u32], spx: usize, x: u32, y: u32, w: u32, h: u32, c: u32) {
    for i in 0..w {
        buf[y as usize * spx + (x + i) as usize] = c;
        buf[(y + h - 1) as usize * spx + (x + i) as usize] = c;
    }
    for j in 0..h {
        buf[(y + j) as usize * spx + x as usize] = c;
        buf[(y + j) as usize * spx + (x + w - 1) as usize] = c;
    }
}

pub(crate) fn panel(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    px: u32,
    py: u32,
    pw: u32,
    ph: u32,
    title: &[u8],
) {
    rect(buf, spx, px, py, pw, ph, BORDER);
    draw_text(buf, spx, w, h, px + 8, py + 6, title, ACCENT);
    for i in 0..pw {
        buf[(py + 22) as usize * spx + (px + i) as usize] = BORDER;
    }
}
