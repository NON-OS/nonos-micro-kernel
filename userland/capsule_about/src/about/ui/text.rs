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
use nonos_toolkit::paint::{measure_ttf, measure_ttf_mono};

fn valid(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes).unwrap_or("")
}

// The rasteriser takes a signed baseline box and drops pixels outside the target,
// so scrolled text needs no clamping of its own: it is placed where the content
// says and the pane's sub-buffer decides how much of it survives. Every placement
// in this capsule goes through here, which is why the top is i32 even for the
// chrome, whose own coordinates are never negative.
pub fn line(fb: &mut PaintBuffer, x: u32, top: i32, bytes: &[u8], argb: u32, px: f32) -> i32 {
    fb.text_ttf(x as i32, top, valid(bytes), argb, px)
}

pub fn mono(fb: &mut PaintBuffer, x: u32, top: i32, bytes: &[u8], argb: u32, px: f32) -> i32 {
    fb.text_ttf_mono(x as i32, top, valid(bytes), argb, px)
}

pub fn width(fb: &PaintBuffer, bytes: &[u8], px: f32) -> u32 {
    fb.measure_ttf(valid(bytes), px).max(0) as u32
}

// The same advance sum without a surface. The click path has no PaintBuffer, so
// any geometry a painter and a hit test share has to be measurable from here.
pub fn width_of(bytes: &[u8], px: f32) -> u32 {
    measure_ttf(valid(bytes), px).max(0) as u32
}

// The same, in the mono face. A column laid out from proportional advances would
// sit left of the mono cells it is meant to align with.
pub fn width_of_mono(bytes: &[u8], px: f32) -> u32 {
    measure_ttf_mono(valid(bytes), px).max(0) as u32
}

pub fn right(fb: &mut PaintBuffer, right_x: u32, top: i32, bytes: &[u8], argb: u32, px: f32) {
    let w = width(fb, bytes, px);
    line(fb, right_x.saturating_sub(w), top, bytes, argb, px);
}

// `text_ttf` takes the top of the line box, so centring one line inside a box
// is the caller's job. Painter and hit test both come through here.
pub fn top_of(y: i32, h: u32, px: f32) -> i32 {
    y + (h.saturating_sub(line_height(px).max(1) as u32) / 2) as i32
}

// A hairline is a rectangle, and rectangles have no negative-y path of their own,
// so a rule that has scrolled out of the pane is simply not drawn.
pub fn rule(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, argb: u32) {
    if y >= 0 && y < fb.height as i32 {
        fb.fill_rect(x, y as u32, w, 1, argb);
    }
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
