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

use super::recents_group::parent_of;
use super::screen_row::{row_text, LINE_BOX, SUB_PX, TITLE_PX};
use super::sidebar_rows::base_label;
use super::theme::PILL_ON;

const HL_PAD: u32 = 3;

/// The first case-insensitive occurrence of `needle` in `hay`, as a byte range
/// that is guaranteed to sit on char boundaries: the start comes from
/// `char_indices` and the end is rejected unless `is_char_boundary` agrees, so a
/// slice of the result can never split a code point.
pub fn find_ci(hay: &str, needle: &str) -> Option<(usize, usize)> {
    if needle.is_empty() {
        return None;
    }
    for (i, _) in hay.char_indices() {
        let end = i + needle.len();
        if end > hay.len() || !hay.is_char_boundary(end) {
            continue;
        }
        if hay[i..end].eq_ignore_ascii_case(needle) {
            return Some((i, end));
        }
    }
    None
}

/// Paints the match band behind a result row's text, before `screen_row` draws
/// the glyphs over it. The store matches on the whole path, so both the title
/// and the parent line are offered the query and whichever contains it lights
/// up; a content-only hit matches neither and is left unbanded.
pub fn highlight(fb: &mut PaintBuffer, x: u32, y: u32, path: &str, query: &str) {
    let (tx, ty, sy) = row_text(x, y);
    let title = base_label(path);
    band(fb, tx, ty, title.as_str(), query, TITLE_PX);
    band(fb, tx, sy, parent_of(path), query, SUB_PX);
}

// The band is placed by measurement alone: the prefix before the match gives its
// x, the matched slice gives its width, so it tracks proportional glyphs instead
// of drifting the way a character count would.
fn band(fb: &mut PaintBuffer, x: u32, y: u32, text: &str, query: &str, px: f32) {
    let Some((a, b)) = find_ci(text, query) else { return };
    let ax = fb.measure_ttf(&text[..a], px).max(0) as u32;
    let w = fb.measure_ttf(&text[a..b], px).max(0) as u32;
    if w == 0 {
        return;
    }
    fb.fill_round(x + ax, y.saturating_sub(HL_PAD), w + HL_PAD, LINE_BOX, HL_PAD, PILL_ON);
}
