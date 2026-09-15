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

// The crate has no shared measured-text helpers of its own; the toolkit exposes
// only `measure_ttf`, and the truncate in `capsule_desktop_shell` is private to
// that capsule. Both cuts below are char-boundary safe by construction: the
// candidate ends always come from `char_indices`.

/// The longest prefix of `text` that measures at or under `max_w` at `px`.
/// Never splits a UTF-8 code point, because every candidate end is a char
/// boundary reported by `char_indices`.
pub fn truncate_to_width<'a>(fb: &PaintBuffer, text: &'a str, px: f32, max_w: u32) -> &'a str {
    if width_of(fb, text, px) <= max_w {
        return text;
    }
    let mut end = 0usize;
    for (i, _) in text.char_indices() {
        if width_of(fb, &text[..i], px) > max_w {
            break;
        }
        end = i;
    }
    &text[..end]
}

/// Draws `text` so its right edge lands on `end_x`, placed by measurement rather
/// than by any glyph-count arithmetic.
pub fn right_text(fb: &mut PaintBuffer, end_x: u32, y: u32, text: &str, px: f32, argb: u32) {
    let w = width_of(fb, text, px);
    let _ = fb.text_ttf(end_x.saturating_sub(w) as i32, y as i32, text, argb, px);
}

/// The measured advance of `text`, clamped into the unsigned space the layout
/// arithmetic works in.
pub fn width_of(fb: &PaintBuffer, text: &str, px: f32) -> u32 {
    fb.measure_ttf(text, px).max(0) as u32
}
