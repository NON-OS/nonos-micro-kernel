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

//! A vertical label list with one selected row. The selection is a rounded
//! accent cap fading right into transparency, so it must be blended over the
//! paint already on the surface rather than written raw.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

use super::truncate::truncate_to_width;

pub(in crate::editor) struct NavStyle {
    pub accent: u32,
    pub ring: u32,
    pub label: u32,
    pub label_sel: u32,
    pub radius: u32,
    pub pad_x: u32,
}

pub(in crate::editor) fn nav_row_h(px: f32) -> u32 {
    line_height(px).max(1) as u32 + 10
}

pub(in crate::editor) fn paint_navlist(
    fb: &mut PaintBuffer,
    rect: (u32, u32, u32),
    labels: &[&str],
    selected: usize,
    px: f32,
    st: &NavStyle,
) {
    let (x, y, w) = rect;
    let rh = nav_row_h(px);
    let lh = line_height(px).max(1) as u32;
    let cap = st.radius.saturating_mul(2).min(w);
    for (i, label) in labels.iter().enumerate() {
        let ry = y + i as u32 * rh;
        if i == selected && w > cap {
            let a = (st.accent >> 24) & 0xFF;
            let mid = (a * (w - cap) as u32 / w) << 24 | (st.accent & 0x00FF_FFFF);
            fb.fill_round(x, ry, cap, rh, st.radius, st.accent);
            fb.gradient_h(x + cap, ry, w - cap, rh, mid, st.accent & 0x00FF_FFFF);
            fb.stroke_round(x, ry, w, rh, st.radius, 1, st.ring);
        }
        let avail = w.saturating_sub(st.pad_x * 2) as i32;
        let cut = truncate_to_width(fb, label, px, avail);
        let color = if i == selected { st.label_sel } else { st.label };
        let ty = (ry + (rh - lh) / 2) as i32;
        let _ = fb.text_ttf((x + st.pad_x) as i32, ty, cut, color, px);
    }
}
