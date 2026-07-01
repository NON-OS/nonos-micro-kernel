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
const DIM: u32 = 0xFF8B_A3AD;

pub(super) fn paint_image(fb: &mut PaintBuffer, s: &Span, y: u32, h: u32) {
    let box_h = h.max(64);
    fb.fill_rect(s.x, y, s.w, box_h, BOX_BG);
    fb.fill_rect(s.x, y, s.w, 1, BOX_EDGE);
    fb.fill_rect(s.x, y + box_h - 1, s.w, 1, BOX_EDGE);
    fb.fill_rect(s.x, y, 1, box_h, BOX_EDGE);
    fb.fill_rect(s.x + s.w - 1, y, 1, box_h, BOX_EDGE);
    fb.fill_rect(s.x + 12, y + 18, 52, 52, 0xFF15_1D22);
    fb.fill_rect(s.x + 22, y + 44, 32, 2, BOX_EDGE);
    fb.fill_rect(s.x + 38, y + 28, 2, 32, BOX_EDGE);
    draw_fit(fb, s.x + 78, y + 18, s.w.saturating_sub(96), &s.text, TEXT);
    draw_fit(fb, s.x + 78, y + 42, s.w.saturating_sub(96), "remote image", DIM);
    if let Some(src) = s.image_src.as_deref() {
        draw_fit(fb, s.x + 78, y + 66, s.w.saturating_sub(96), src, BOX_EDGE);
    }
}

fn draw_fit(fb: &mut PaintBuffer, x: u32, y: u32, max_w: u32, text: &str, color: u32) {
    let adv = fb.glyph_advance().max(1);
    let max = (max_w / adv) as usize;
    let mut count = 0;
    let mut end = text.len();
    for (idx, _) in text.char_indices() {
        if count == max {
            end = idx;
            break;
        }
        count += 1;
    }
    fb.text(x, y, &text.as_bytes()[..end], color);
}
