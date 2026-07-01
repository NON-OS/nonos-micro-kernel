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

use crate::browser::layout::doc::Span;

const BOX_BG: u32 = 0xFF20_2A30;
const BOX_EDGE: u32 = 0xFF46_A6B2;
const TEXT: u32 = 0xFFD7_FCFF;

pub(super) fn paint_image(fb: &mut PaintBuffer, s: &Span, y: u32, h: u32) {
    let box_h = h.max(64);
    fb.fill_rect(s.x, y, s.w, box_h, BOX_BG);
    fb.fill_rect(s.x, y, s.w, 1, BOX_EDGE);
    fb.fill_rect(s.x, y + box_h - 1, s.w, 1, BOX_EDGE);
    fb.fill_rect(s.x, y, 1, box_h, BOX_EDGE);
    fb.fill_rect(s.x + s.w - 1, y, 1, box_h, BOX_EDGE);
    fb.text(s.x + 8, y + 10, s.text.as_bytes(), TEXT);
    if let Some(src) = s.image_src.as_deref() {
        fb.text(s.x + 8, y + 30, src.as_bytes(), BOX_EDGE);
    }
}
