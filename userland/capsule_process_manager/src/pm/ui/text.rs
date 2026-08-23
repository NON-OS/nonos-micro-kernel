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
use nonos_toolkit::font::ttf::line_height;

fn valid(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes).unwrap_or("")
}

pub fn left(fb: &mut PaintBuffer, x: u32, top: u32, bytes: &[u8], argb: u32, px: f32) -> i32 {
    fb.text_ttf(x as i32, top as i32, valid(bytes), argb, px)
}

pub fn mono(fb: &mut PaintBuffer, x: u32, top: u32, bytes: &[u8], argb: u32, px: f32) -> i32 {
    fb.text_ttf_mono(x as i32, top as i32, valid(bytes), argb, px)
}

pub fn width(fb: &PaintBuffer, bytes: &[u8], px: f32) -> u32 {
    fb.measure_ttf(valid(bytes), px).max(0) as u32
}

pub fn mono_width(fb: &PaintBuffer, bytes: &[u8], px: f32) -> u32 {
    fb.measure_ttf_mono(valid(bytes), px).max(0) as u32
}

// A column of numbers only reads as a column when the last digit lines up, so
// numeric cells are placed from their right edge rather than their left.
pub fn mono_right(fb: &mut PaintBuffer, right_x: u32, top: u32, bytes: &[u8], argb: u32, px: f32) {
    let w = mono_width(fb, bytes, px);
    mono(fb, right_x.saturating_sub(w), top, bytes, argb, px);
}

pub fn right(fb: &mut PaintBuffer, right_x: u32, top: u32, bytes: &[u8], argb: u32, px: f32) {
    let w = width(fb, bytes, px);
    left(fb, right_x.saturating_sub(w), top, bytes, argb, px);
}

// `text_ttf` takes the top of the line box, so centring one line inside a box
// is the caller's job. Painter and hit test both come through here.
pub fn centred_top(y: u32, h: u32, px: f32) -> u32 {
    let lh = line_height(px).max(1) as u32;
    y + h.saturating_sub(lh) / 2
}

// Longest char-boundary-safe prefix that measures within `max_w`. Never cut by
// glyph count here: the body face is proportional.
pub fn fit<'a>(fb: &PaintBuffer, bytes: &'a [u8], px: f32, max_w: u32) -> &'a [u8] {
    let s = valid(bytes);
    let mut end = s.len();
    while end > 0 {
        if fb.measure_ttf(&s[..end], px).max(0) as u32 <= max_w {
            return &bytes[..end];
        }
        end -= 1;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
    }
    &bytes[..0]
}
