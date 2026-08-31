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

//! Measured, char-boundary-safe truncation. Proportional glyphs make any
//! `len() * 8` estimate wrong, so the cut is found by measuring prefixes.

use nonos_app_skeleton::PaintBuffer;

pub(in crate::editor) fn truncate_to_width<'a>(
    fb: &PaintBuffer,
    text: &'a str,
    px: f32,
    max_w: i32,
) -> &'a str {
    if max_w <= 0 {
        return "";
    }
    if fb.measure_ttf(text, px) <= max_w {
        return text;
    }
    let mut end = 0usize;
    for (i, c) in text.char_indices() {
        let next = i + c.len_utf8();
        if fb.measure_ttf(&text[..next], px) > max_w {
            break;
        }
        end = next;
    }
    &text[..end]
}
