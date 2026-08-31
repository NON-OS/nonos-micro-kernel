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

use nonos_app_skeleton::paint::PaintBuffer;

const TIP: u32 = 3;

fn run_width(row: u32, h: u32) -> u32 {
    let half = if row * 2 <= h { row } else { h.saturating_sub(row) };
    TIP + half
}

pub fn triangle_right(fb: &mut PaintBuffer, x: u32, y: u32, h: u32, argb: u32) {
    for row in 0..h {
        fb.fill_rect(x, y + row, run_width(row, h), 1, argb);
    }
}

pub fn triangle_left(fb: &mut PaintBuffer, x: u32, y: u32, h: u32, argb: u32) {
    let span = TIP + h / 2;
    for row in 0..h {
        let w = run_width(row, h);
        fb.fill_rect(x + span.saturating_sub(w), y + row, w, 1, argb);
    }
}

pub fn pause(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, gap: u32, argb: u32) {
    fb.fill_rect(x, y, w, h, argb);
    fb.fill_rect(x + w + gap, y, w, h, argb);
}
