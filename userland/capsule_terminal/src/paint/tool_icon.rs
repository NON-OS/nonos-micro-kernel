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

use crate::layout::Rect;

fn center(r: Rect) -> (u32, u32, u32) {
    let s = (r.w.min(r.h) / 2).max(2);
    (r.x + r.w / 2, r.y + r.h / 2, s)
}

pub fn icon_new_tab(fb: &mut PaintBuffer, r: Rect, argb: u32) {
    let (cx, cy, s) = center(r);
    fb.ring(cx, cy, s, 1, argb);
    fb.blend_rect(cx - s / 2, cy, s, 1, argb);
    fb.blend_rect(cx, cy - s / 2, 1, s, argb);
}

pub fn icon_split(fb: &mut PaintBuffer, r: Rect, argb: u32) {
    let (cx, cy, s) = center(r);
    fb.stroke_round(cx - s, cy - s + 1, s * 2, s * 2 - 2, 2, 1, argb);
    fb.blend_rect(cx, cy - s + 2, 1, s * 2 - 4, argb);
}

pub fn icon_search(fb: &mut PaintBuffer, r: Rect, argb: u32) {
    let (cx, cy, s) = center(r);
    fb.ring(cx - 1, cy - 1, s - 1, 1, argb);
    let a = (cx + s / 2) as i32;
    let b = (cy + s / 2) as i32;
    fb.line_aa(a, b, a + s as i32 / 2, b + s as i32 / 2, argb);
}

pub fn icon_theme(fb: &mut PaintBuffer, r: Rect, argb: u32) {
    let (cx, cy, s) = center(r);
    fb.ring(cx, cy, s, 1, argb);
    fb.circle(cx, cy - s / 2, 1, argb);
    fb.circle(cx - s / 2, cy + s / 3, 1, argb);
    fb.circle(cx + s / 2, cy + s / 3, 1, argb);
}

pub fn icon_settings(fb: &mut PaintBuffer, r: Rect, argb: u32) {
    let (cx, cy, s) = center(r);
    let h = s / 2 + 1;
    fb.ring(cx, cy, h, 1, argb);
    fb.blend_rect(cx - s, cy, s - h, 1, argb);
    fb.blend_rect(cx + h + 1, cy, s - h, 1, argb);
    fb.blend_rect(cx, cy - s, 1, s - h, argb);
    fb.blend_rect(cx, cy + h + 1, 1, s - h, argb);
}
