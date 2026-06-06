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
pub fn fill_rect(
    buf: &mut [u32],
    stride: usize,
    fb_w: u32,
    fb_h: u32,
    rect: (u32, u32, u32, u32),
    color: u32,
) {
    let (x, y, w, h) = rect;
    let x0 = x.min(fb_w) as usize;
    let y0 = y.min(fb_h) as usize;
    let x1 = (x.saturating_add(w)).min(fb_w) as usize;
    let y1 = (y.saturating_add(h)).min(fb_h) as usize;
    let mut py = y0;
    while py < y1 {
        let row = py.saturating_mul(stride);
        let mut px = x0;
        while px < x1 {
            let i = row.saturating_add(px);
            if i < buf.len() {
                buf[i] = color;
            }
            px += 1;
        }
        py += 1;
    }
}
