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

// A power-up is a diamond, and the platform has no rotated fill, so the shape
// is scanned one row at a time and outlined with the antialiased polyline.
pub fn fill(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32, argb: u32) {
    let r = r.max(1);
    for dy in 0..=(r * 2) {
        let offset = if dy > r { dy - r } else { r - dy };
        let half = r - offset;
        let y = (cy + dy).saturating_sub(r);
        fb.blend_rect(cx.saturating_sub(half), y, half * 2 + 1, 1, argb);
    }
}

pub fn ring(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32, argb: u32) {
    let (x, y, r) = (cx as i32, cy as i32, r.max(1) as i32);
    fb.polyline_aa(&[(x, y - r), (x + r, y), (x, y + r), (x - r, y), (x, y - r)], argb);
}
