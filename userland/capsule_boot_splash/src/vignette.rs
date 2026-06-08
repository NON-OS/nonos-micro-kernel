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

const CORE: u32 = 0xFF06_201C;
const BG: u32 = 0xFF00_0000;

pub(crate) fn fill(buf: &mut [u32], spx: usize, w: u32, h: u32) {
    fill_band(buf, spx, w, h, 0, h);
}

pub(crate) fn fill_band(buf: &mut [u32], spx: usize, w: u32, h: u32, y0: u32, rows: u32) {
    let cx = w as i64 / 2;
    let cy = h as i64 * 38 / 100;
    let r2 = ((w as i64 * w as i64) / 3).max(1);
    for y in y0 as i64..(y0 + rows) as i64 {
        for x in 0..w as i64 {
            let dx = x - cx;
            let dy = (y - cy) * 12 / 10;
            let t = (((dx * dx + dy * dy) * 256 / r2).min(256)) as u32;
            buf[y as usize * spx + x as usize] = lerp(CORE, BG, t);
        }
    }
}

fn lerp(a: u32, b: u32, t: u32) -> u32 {
    let mut out = 0xFF00_0000;
    let mut s = 0u32;
    while s < 24 {
        let c = (((a >> s) & 0xFF) * (256 - t) + ((b >> s) & 0xFF) * t) / 256;
        out |= (c & 0xFF) << s;
        s += 8;
    }
    out
}
