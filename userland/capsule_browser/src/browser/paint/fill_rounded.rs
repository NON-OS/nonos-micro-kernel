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

use nonos_app_skeleton::PaintBuffer;

use super::fill_page::fill_page;

// Rounded rect: one full-width band between the corner regions, then per-row
// spans whose inset follows the corner circle.
pub(super) fn fill_rounded(
    fb: &mut PaintBuffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: u32,
    color: u32,
    clip: Option<[i32; 4]>,
) {
    let r = (radius as i32).min(w / 2).min(h / 2).min(64);
    if r <= 0 {
        fill_page(fb, x, y, w, h, color, clip);
        return;
    }
    fill_page(fb, x, y + r, w, h - 2 * r, color, clip);
    for row in 0..r {
        let dyc = (r - row - 1) as i64;
        let rr = r as i64;
        let mut span = 0i64;
        while (span + 1) * (span + 1) + dyc * dyc <= rr * rr {
            span += 1;
        }
        let inset = r - span as i32;
        fill_page(fb, x + inset, y + row, w - 2 * inset, 1, color, clip);
        fill_page(fb, x + inset, y + h - 1 - row, w - 2 * inset, 1, color, clip);
    }
}
