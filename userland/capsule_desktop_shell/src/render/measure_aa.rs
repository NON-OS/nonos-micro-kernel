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

//! The desktop's antialiased measure path. Widths and truncation come from the
//! face itself rather than a fixed advance, so proportional glyphs stop being
//! sized as if every character were eight pixels wide.

use nonos_toolkit::font::ttf;

use super::ui_font::valid_str;

/// Rendered width of `text` at `px`.
pub fn measure_aa(text: &str, px: f32) -> u32 {
    ttf::measure(text, super::ui_font::scaled(px), false).max(0) as u32
}

/// Rendered width of `text` at `px` in the bold face.
pub fn measure_aa_bold(text: &str, px: f32) -> u32 {
    match ttf::builtin_face(false, true) {
        Some(face) => ttf::measure_with(face, text, super::ui_font::scaled(px)).max(0) as u32,
        None => measure_aa(text, px),
    }
}

/// Rendered width of a byte-slice label at `px`.
pub fn measure_aa_bytes(bytes: &[u8], px: f32) -> u32 {
    measure_aa(valid_str(bytes), px)
}

/// The longest prefix of `text` that fits in `max_w`. Cuts on a char boundary, so a
/// truncated label is short rather than corrupt.
pub fn truncate_to_width(text: &str, px: f32, max_w: u32) -> &str {
    if measure_aa(text, px) <= max_w {
        return text;
    }
    let mut fit = 0;
    for (i, _) in text.char_indices() {
        if measure_aa(&text[..i], px) > max_w {
            break;
        }
        fit = i;
    }
    &text[..fit]
}
