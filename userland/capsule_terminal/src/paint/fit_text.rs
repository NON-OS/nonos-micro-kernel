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

/// Longest char-boundary-safe prefix of `text` that measures within `max_w` at
/// `px`, so callers cut proportional glyphs by width instead of by count.
pub fn fit_text<'a>(fb: &PaintBuffer<'_>, text: &'a str, px: f32, max_w: u32) -> &'a str {
    if width_of(fb, text, px) <= max_w {
        return text;
    }
    let mut end = text.len();
    while end > 0 {
        end -= 1;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        if width_of(fb, &text[..end], px) <= max_w {
            break;
        }
    }
    &text[..end]
}

/// Measured advance width of `text` at `px`, clamped to a pixel count.
pub fn width_of(fb: &PaintBuffer<'_>, text: &str, px: f32) -> u32 {
    fb.measure_ttf(text, px).max(0) as u32
}
