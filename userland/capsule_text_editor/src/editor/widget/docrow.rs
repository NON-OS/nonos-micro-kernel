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

//! One document entry: a rounded icon block with a page mark drawn from
//! primitives, a title line, and a dimmed subtitle beneath it. Both lines are
//! measured against the width left over after the icon.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

use super::truncate::truncate_to_width;

pub(in crate::editor) struct DocRowStyle {
    pub icon_bg: u32,
    pub icon_mark: u32,
    pub icon_radius: u32,
    pub title: u32,
    pub subtitle: u32,
    pub gap: u32,
}

pub(in crate::editor) fn paint_docrow(
    fb: &mut PaintBuffer,
    rect: (u32, u32, u32, u32),
    icon: u32,
    lines: (&str, &str),
    px: (f32, f32),
    st: &DocRowStyle,
) {
    let (x, y, w, h) = rect;
    let (title, subtitle) = lines;
    let (title_px, sub_px) = px;
    let iy = y + h.saturating_sub(icon) / 2;
    fb.fill_round(x, iy, icon, icon, st.icon_radius, st.icon_bg);
    paint_page_mark(fb, x, iy, icon, st.icon_mark);
    let tx = x + icon + st.gap;
    let avail = (x + w).saturating_sub(tx) as i32;
    let th = line_height(title_px).max(1) as u32;
    let sh = line_height(sub_px).max(1) as u32;
    let ty = y + h.saturating_sub(th + sh) / 2;
    let cut = truncate_to_width(fb, title, title_px, avail);
    let _ = fb.text_ttf(tx as i32, ty as i32, cut, st.title, title_px);
    let cut = truncate_to_width(fb, subtitle, sub_px, avail);
    let _ = fb.text_ttf(tx as i32, (ty + th) as i32, cut, st.subtitle, sub_px);
}

fn paint_page_mark(fb: &mut PaintBuffer, x: u32, y: u32, icon: u32, argb: u32) {
    let inset = (icon / 4).max(2);
    let mw = icon.saturating_sub(inset * 2);
    let bar = (icon / 12).max(1);
    if mw == 0 {
        return;
    }
    for row in 0..3u32 {
        let by = y + inset + row * (bar * 2 + 1);
        if by + bar > y + icon - inset {
            break;
        }
        let rw = if row == 2 { mw * 2 / 3 } else { mw };
        fb.blend_rect(x + inset, by, rw.max(1), bar, argb);
    }
}
