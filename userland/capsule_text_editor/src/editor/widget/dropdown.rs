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

//! The closed half of a dropdown: a rounded bordered box holding the current
//! value with a chevron at its right edge. The open popup is the menubar and
//! ribbon panel machinery, which the screens reuse.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

use super::truncate::truncate_to_width;

pub(in crate::editor) struct DropdownStyle {
    pub bg: u32,
    pub border: u32,
    pub radius: u32,
    pub text: u32,
    pub chevron: u32,
    pub pad_x: u32,
}

pub(in crate::editor) fn dropdown_w(
    fb: &PaintBuffer,
    value: &str,
    px: f32,
    min_w: u32,
    st: &DropdownStyle,
) -> u32 {
    let text = fb.measure_ttf(value, px).max(0) as u32;
    (text + st.pad_x * 3 + chevron_w()).max(min_w)
}

pub(in crate::editor) fn paint_dropdown(
    fb: &mut PaintBuffer,
    rect: (u32, u32, u32, u32),
    value: &str,
    px: f32,
    st: &DropdownStyle,
) {
    let (x, y, w, h) = rect;
    fb.panel(x, y, w, h, st.radius, st.bg, st.border);
    let lh = line_height(px).max(1) as u32;
    let ty = (y + h.saturating_sub(lh) / 2) as i32;
    let avail = w.saturating_sub(st.pad_x * 3 + chevron_w()) as i32;
    let cut = truncate_to_width(fb, value, px, avail);
    let _ = fb.text_ttf((x + st.pad_x) as i32, ty, cut, st.text, px);
    paint_chevron(fb, x + w.saturating_sub(st.pad_x + chevron_w()), y + h / 2, st.chevron);
}

fn chevron_w() -> u32 {
    9
}

fn paint_chevron(fb: &mut PaintBuffer, x: u32, cy: u32, argb: u32) {
    for i in 0..4u32 {
        fb.blend_rect(x + i, cy.saturating_sub(2) + i, 2, 2, argb);
        fb.blend_rect(x + 7 - i, cy.saturating_sub(2) + i, 2, 2, argb);
    }
}
