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

//! A read-only search field: rounded panel, a leading glyph slot, and either
//! the current text or a dimmed placeholder. Key handling belongs to the
//! screen that owns the field.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

use super::truncate::truncate_to_width;

pub(in crate::editor) struct SearchStyle {
    pub bg: u32,
    pub border: u32,
    pub radius: u32,
    pub text: u32,
    pub placeholder: u32,
    pub glyph: u32,
    pub pad_x: u32,
    pub gap: u32,
}

pub(in crate::editor) fn paint_searchbox(
    fb: &mut PaintBuffer,
    rect: (u32, u32, u32, u32),
    glyph: &str,
    text: &str,
    placeholder: &str,
    px: f32,
    st: &SearchStyle,
) {
    let (x, y, w, h) = rect;
    fb.panel(x, y, w, h, st.radius, st.bg, st.border);
    let lh = line_height(px).max(1) as u32;
    let ty = (y + h.saturating_sub(lh) / 2) as i32;
    let gw = fb.measure_ttf(glyph, px).max(0) as u32;
    let _ = fb.text_ttf((x + st.pad_x) as i32, ty, glyph, st.glyph, px);
    let tx = x + st.pad_x + gw + st.gap;
    let avail = (x + w).saturating_sub(tx + st.pad_x) as i32;
    let (body, color) = match text.is_empty() {
        true => (placeholder, st.placeholder),
        false => (text, st.text),
    };
    let cut = truncate_to_width(fb, body, px, avail);
    let _ = fb.text_ttf(tx as i32, ty, cut, color, px);
}

pub(in crate::editor) fn searchbox_hit(rect: (u32, u32, u32, u32), mx: i32, my: i32) -> bool {
    let (x, y, w, h) = rect;
    mx >= x as i32 && my >= y as i32 && mx < (x + w) as i32 && my < (y + h) as i32
}
