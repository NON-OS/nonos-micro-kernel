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

use super::icon_glyph::draw_glyph;
use super::icon_search::SEARCH;
use super::metrics::{HEAD_ICON, PANE_PAD_X, SUBTITLE_PX, TITLE_PX};
use super::text;
use super::theme::{ACCENT, SUBTITLE_FG, TITLE_FG};
use super::valbuf::ValBuf;

pub fn paint(fb: &mut PaintBuffer, query: &str, n: usize, top: i32) {
    let x = PANE_PAD_X;
    if top >= 0 {
        draw_glyph(fb, &SEARCH, x, top as u32, HEAD_ICON, ACCENT);
    }
    let text_x = x + HEAD_ICON + 14;
    text::left(fb, text_x, top, "Search", TITLE_FG, TITLE_PX);
    let below = top + line_height(TITLE_PX) + 2;
    let mut sub = ValBuf::new();
    summary(&mut sub, query, n);
    text::left(fb, text_x, below, sub.as_str(), SUBTITLE_FG, SUBTITLE_PX);
}

fn summary(sub: &mut ValBuf, query: &str, n: usize) {
    if n == 0 {
        sub.push_str("No setting matches \"");
        sub.push_str(query);
        sub.push_str("\".");
        return;
    }
    sub.push_u32(n as u32);
    sub.push_str(if n == 1 { " setting matches \"" } else { " settings match \"" });
    sub.push_str(query);
    sub.push_str("\".");
}
