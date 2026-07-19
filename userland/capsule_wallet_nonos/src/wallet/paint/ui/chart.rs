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

// A column bar chart. Each value is a 0..100 height percentage of `h`.
pub fn bars(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, vals: &[u8], color: u32) {
    let n = vals.len() as u32;
    if n == 0 || w == 0 {
        return;
    }
    let gap = 5u32;
    let bw = (w.saturating_sub(gap * (n - 1)) / n).max(1);
    for (i, &v) in vals.iter().enumerate() {
        let bh = (v as u32 * h / 100).max(1);
        let bx = x + i as u32 * (bw + gap);
        fb.fill_rect(bx, y + h - bh, bw, bh, color);
    }
}
