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

//! The pane header: greeting, strapline, and the search field. The field is
//! display-only until there is an index to search, so it is drawn with a sunk
//! glyph and placeholder.

use nonos_app_skeleton::PaintBuffer;

use crate::editor::widget::{paint_searchbox, truncate_to_width, SearchStyle};

use super::metrics::{lh, BODY, HEAD, PANE_PAD};
use super::metrics_pane::{pane_content, search_rect};
use super::palette::{dim, FIELD_BG, FIELD_LINE, LABEL, MUTED, TITLE};

const GREETING: &str = "Welcome back, Mehedi";
const STRAPLINE: &str = "Create, open, and manage your documents.";

pub(super) fn paint_pane_head(fb: &mut PaintBuffer) {
    let (x, w) = pane_content(fb.width);
    let head = truncate_to_width(fb, GREETING, HEAD, w as i32);
    let _ = fb.text_ttf(x as i32, PANE_PAD as i32, head, TITLE, HEAD);
    let sub_y = (PANE_PAD + lh(HEAD) + 4) as i32;
    let sub = truncate_to_width(fb, STRAPLINE, BODY, w as i32);
    let _ = fb.text_ttf(x as i32, sub_y, sub, LABEL, BODY);
    let st = SearchStyle {
        bg: FIELD_BG,
        border: FIELD_LINE,
        radius: 10,
        text: TITLE,
        placeholder: dim(MUTED),
        glyph: dim(MUTED),
        pad_x: 14,
        gap: 8,
    };
    let rect = search_rect(fb.width);
    paint_searchbox(fb, rect, "⌕", "", "Search documents...", BODY, &st);
}
